# Architecture

## System Overview

Commando v1 follows a modular, component-based architecture with clear separation of concerns. The system is designed for high cohesion within modules and low coupling between them, making it maintainable, testable, and extensible.

```mermaid
graph TB
    subgraph "Component Architecture"
        CLI[CLI Controller] --> SC[Staging Checker]
        CLI --> IH[Input Handler]
        IH --> V[Validator]
        V --> CB[CommitBuffer]
        CB --> PR[Preview Renderer]
        CB --> E[Editor]
        CB --> CE[Commit Executor]
        PR --> CLI
        E --> V
        CE --> Git[Git Repository]
    end
```

## Components Specifications

### CLI Controller

**Responsibilities**
- Orchestrates the full workflow
- Checks staging first
- Delegates to input, validation, preview, edit, commit

**Inputs**
- Command-line invocation (`commando commit`)
- Optional config / repo path

**Outputs**
- Success/failure messages
- Exit code (0 for success, non-zero for failure)

**Interactions**
- Calls **Staging Checker** first
- If staged → calls Input Handler → Validator → CommitBuffer → Preview Renderer → Editor → Commit Executor
- Handles control flow looping for edit & preview

---

### Staging Checker

**Responsibilities**
- Determine if there are staged changes in the repo
- Provide list of staged files (optional)

**Inputs**
- Git repository path
- Optional flags (e.g., include untracked files?)

**Outputs**
- `Bool` → true if staged changes exist
- `List<String>` → staged files (optional)
- Error if repo not found

**Interactions**
- Returns status to **CLI Controller**
- Prevents commit workflow if no staged changes

---

### Input Handler

**Responsibilities**
- Prompt user for required and optional fields
- Capture input while enforcing type rules

**Inputs**
- Predefined list of allowed types (`feat`, `fix`, …)
- Optional prompts (scope, details, breaking changes)

**Outputs**
- Raw input for each field (to feed Validator / CommitBuffer)

**Interactions**
- Feeds captured data into **Validator**
- Can loop back from **Editor** for re-entry

---

### Validator

**Responsibilities**
- Enforce rules on required and optional fields
- Type must be in allowed list
- Required fields: non-empty, defined
- Optional fields: undefined or non-empty string

**Inputs**
- Raw input from Input Handler / Editor

**Outputs**
- Validation status (`Bool`)
- Errors / messages if invalid

**Interactions**
- Sends feedback to CLI Controller for user correction
- Only validated data stored in **CommitBuffer**

---

### CommitBuffer / Data Model

**Responsibilities**
- Store commit data in structured way
- Provide serialized commit string for preview & commit
- Mutable during edit phase

**Inputs**
- Validated fields from Validator
- Updates from Editor

**Outputs**
- Formatted commit string for preview or commit
- Exposed getters for individual fields (type, description, scope…)

**Interactions**
- Central hub: read/write by Input Handler, Editor, Preview Renderer, Commit Executor

---

### Preview Renderer

**Responsibilities**
- Format commit message for user preview
- Display required + optional fields clearly

**Inputs**
- CommitBuffer (current state)

**Outputs**
- Formatted string to display
- Optional color or highlighting for text-based CLI

**Interactions**
- Invoked by CLI Controller before final confirmation
- Invoked again after editing fields

---

### Editor

**Responsibilities**
- Allow user to edit any field before committing
- Validate changes (via Validator)

**Inputs**
- CommitBuffer
- User selection of field(s) to edit

**Outputs**
- Updated CommitBuffer
- Validation feedback if needed

**Interactions**
- Loops back to **Preview Renderer**
- Only passes back to CLI Controller after user confirms edits

---

### Commit Executor

**Responsibilities**
- Write commit message to Git repo
- Ensure commit respects enforced structure
- Report success/failure

**Inputs**
- CommitBuffer (validated & confirmed)
- Git repository context

**Outputs**
- Success/failure message
- Git commit status code

**Interactions**
- Called once CLI Controller gets final confirmation

---

## Data Flow Architecture

### Primary Flow

```mermaid
sequenceDiagram
    participant User
    participant CLI as CLI Controller
    participant SC as Staging Checker
    participant IH as Input Handler
    participant V as Validator
    participant CB as CommitBuffer
    participant PR as Preview Renderer
    participant E as Editor
    participant CE as Commit Executor
    participant Git as Git Repository

    User->>CLI: commando commit
    CLI->>SC: Check for staged changes
    SC-->>CLI: Staged changes exist

    loop Until validated
        CLI->>IH: Prompt for fields
        IH-->>CLI: Raw input
        CLI->>V: Validate input
        V-->>CLI: Validation result
    end

    CLI->>CB: Store validated data
    CLI->>PR: Show preview
    PR-->>User: Display formatted commit

    alt User wants edit
        User->>CLI: Request edit
        loop Edit workflow
            CLI->>E: Edit fields
            E->>V: Validate changes
            V-->>E: Validation
            E->>CB: Update data
            CLI->>PR: Show updated preview
            PR-->>User: Display preview
        end
    end

    User->>CLI: Confirm commit
    CLI->>CE: Execute commit
    CE->>Git: git commit -m "..."
    Git-->>CE: Commit result
    CE-->>User: Success/Error message
    CLI-->>User: Exit with code
```

### Execution Flow

```mermaid
flowchart TD
    Start([User runs commando commit]) --> CLI

    subgraph CLI["CLI Controller"]
        direction TB
        C1[Orchestrate workflow] --> C2[Control flow manager]
        C2 --> C3[Exit code handler]
    end

    CLI --> SC{Staging Checker}
    SC -->|No staged changes| Error[Exit: No staged changes]
    SC -->|Staged changes exist| IH

    subgraph IH["Input Handler"]
        direction TB
        IH1[Prompt for type] --> IH2[Prompt for description]
        IH2 --> IH3[Prompt for scope optional]
        IH3 --> IH4[Prompt for details optional]
        IH4 --> IH5[Prompt for breaking changes optional]
    end

    IH --> V

    subgraph V[Validator]
        direction TB
        V1{Type in allowed list?} -->|No| V2[Validation Error]
        V1 -->|Yes| V3{Description non-empty?}
        V3 -->|No| V2
        V3 -->|Yes| V4{Optional fields valid?}
        V4 -->|Yes| V5[Validation Pass]
        V4 -->|No| V2
    end

    V2 -->|Re-prompt| IH
    V5 --> CB

    subgraph CB["CommitBuffer / Data Model"]
        CB1[Store validated data] --> CB2[Serializable structure]
        CB2 --> CB3[Getters for fields]
    end

    CB --> PR

    subgraph PR["Preview Renderer"]
        direction TB
        PR1[Format commit message] --> PR2[Add color/highlighting]
        PR2 --> PR3[Display to user]
    end

    PR --> Confirm{User confirm?}
    Confirm -->|No, need edit| E
    Confirm -->|Yes| CE

    subgraph E[Editor]
        direction TB
        E1[Select field to edit] --> E2[Edit field value]
        E2 --> E3[Validate via Validator]
        E3 -->|Valid| E4[Update CommitBuffer]
        E3 -->|Invalid| E2
    end

    E --> PR

    subgraph CE["Commit Executor"]
        direction TB
        CE1[Create Git commit] --> CE2[Use formatted message]
        CE2 --> CE3[Execute git commit -m]
        CE3 --> CE4{Commit success?}
        CE4 -->|Yes| CE5[Success message]
        CE4 -->|No| CE6[Error message]
    end

    CE5 --> EndSuccess([Success Exit 0])
    CE6 --> EndError([Error Exit 1])
    Error --> EndError

    %% Styling
    classDef controller fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    classDef data fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef io fill:#f1f8e9,stroke:#33691e,stroke-width:2px
    classDef validation fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef git fill:#ffebee,stroke:#b71c1c,stroke-width:2px

    class CLI,IH,PR,E controller
    class CB data
    class SC,CE io
    class V validation
    class CE git
```

### Error Flow

```mermaid
graph LR
    Start[Start] --> CheckStaging
    CheckStaging -->|No staged changes| Error1[Exit: No changes]
    CheckStaging -->|Error| Error2[Exit: Git error]

    CheckStaging -->|Success| CollectInput
    CollectInput -->|Invalid type| RetryType[Re-prompt type]
    CollectInput -->|Empty description| RetryDesc[Re-prompt description]

    CollectInput -->|Valid| ValidateAll
    ValidateAll -->|Validation failed| ShowErrors[Display errors]
    ShowErrors --> CollectInput

    ValidateAll -->|Valid| Continue[Continue workflow]
```

## Technical Decisions

### 1. Modular Design Rationale

- **Separation of Concerns:** Each component has single responsibility
- **Testability:** Independent components are easier to test
- **Maintainability:** Changes are isolated to specific modules
- **Extensibility:** New features can be added with minimal impact

### 2. Data Management Strategy

- **Centralized State:** CommitBuffer serves as single source of truth
- **Immutable Operations:** Validation creates new validated objects
- **Serializable Format:** Easy conversion to Git commit format

### 3. Error Handling Approach

- **Fail-Fast:** Validate early and often
- **Recovery-Oriented:** Allow correction without restarting
- **Informative Errors:** Provide actionable error messages

### 4. Performance Considerations

- **Lazy Loading:** Load Git data only when needed
- **Caching:** Cache validation results during edit
- **Optimized Rendering:** Only re-render changed fields

## Dependencies

### External Dependencies

- Git command-line tool (system dependency)
- Terminal/console API
- File system access

### Internal Dependencies

```mermaid
graph TD
    CLI --> SC
    CLI --> IH
    IH --> V
    V --> CB
    CB --> PR
    CB --> E
    CB --> CE
    E --> V
    PR --> CLI

    style CLI fill:#e1f5fe
    style SC fill:#f3e5f5
    style IH fill:#fff3e0
    style V fill:#f1f8e9
    style CB fill:#ffebee
    style PR fill:#e8f5e8
    style E fill:#fff8e1
    style CE fill:#fce4ec
```

## Scalability Considerations

### Horizontal Scalability

- Component independence allows parallel development
- Clear interfaces enable team collaboration
- Modular testing supports continuous integration

### Vertical Scalability

- Additional commit fields can be added
- New validation rules can be implemented
- Multiple output formats can be supported
- Plugin system for extensibility

## Security Considerations

- Input sanitization for Git commands
- Path traversal prevention
- Permission validation for Git operations
- Secure temporary file handling

## Accessibility Considerations

1. Keyboard navigation support
2. Screen reader compatibility
3. Color contrast requirements
4. Clear focus indicators
