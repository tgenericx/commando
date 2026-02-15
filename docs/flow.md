# Flow Diagrams

This document contains detailed flow diagrams showing the processes and state transitions within Commando.

## Table of Contents

- [Application Flow](#application-flow)
- [Input Mode Flows](#input-mode-flows)
- [Git Operations Flow](#git-operations-flow)
- [Compilation Flow](#compilation-flow)
- [State Machines](#state-machines)
- [Error Handling Flow](#error-handling-flow)

## Application Flow

### Main Application Flow

```mermaid
flowchart TD
    Start([Start]) --> Init[Initialize Application]
    Init --> ParseArgs[Parse CLI Arguments]
    ParseArgs --> ValidateEnv{Valid Environment?}
    
    ValidateEnv -->|No Git| ErrNoGit[Error: Git Not Found]
    ValidateEnv -->|No Repo| ErrNoRepo[Error: Not a Git Repository]
    ValidateEnv -->|Valid| LoadConfig[Load Configuration]
    
    LoadConfig --> DetermineMode{Determine Input Mode}
    
    DetermineMode -->|--interactive| CreateInteractive[Create Interactive Input]
    DetermineMode -->|--editor| CreateEditor[Create Editor Input]
    DetermineMode -->|-m flag| CreateDirect[Create Direct Input]
    DetermineMode -->|default| CreateInteractive
    
    CreateInteractive --> CollectInput[Collect Input]
    CreateEditor --> CollectInput
    CreateDirect --> CollectInput
    
    CollectInput --> ValidateMsg{Valid Message?}
    
    ValidateMsg -->|Invalid| ShowErrors[Show Validation Errors]
    ShowErrors --> Retry{Retry?}
    Retry -->|Yes| CollectInput
    Retry -->|No| Cancel
    
    ValidateMsg -->|Valid| CheckStaging[Check Staging Area]
    
    CheckStaging --> HasChanges{Has Staged Changes?}
    
    HasChanges -->|No| WarnNoChanges[Warn: No Changes]
    WarnNoChanges --> OfferStage{Stage All?}
    OfferStage -->|Yes| StageAll[Stage All Changes]
    OfferStage -->|No| Cancel
    StageAll --> Preview
    
    HasChanges -->|Yes| Preview[Preview Message]
    
    Preview --> Confirm{Confirm Commit?}
    
    Confirm -->|No| OfferEdit{Edit?}
    OfferEdit -->|Yes| CollectInput
    OfferEdit -->|No| Cancel
    
    Confirm -->|Yes| ExecuteCommit[Execute Git Commit]
    
    ExecuteCommit --> CommitSuccess{Success?}
    
    CommitSuccess -->|Yes| ShowSuccess[Show Success Message]
    CommitSuccess -->|No| ShowCommitError[Show Error]
    
    ShowSuccess --> DisplayHash[Display Commit Hash]
    DisplayHash --> End([End])
    
    ShowCommitError --> Retry2{Retry?}
    Retry2 -->|Yes| ExecuteCommit
    Retry2 -->|No| Cancel
    
    Cancel[Cancel Commit] --> End
    ErrNoGit --> End
    ErrNoRepo --> End
    
    style Start fill:#90EE90
    style End fill:#FFB6C1
    style Cancel fill:#FFB6C1
    style ExecuteCommit fill:#87CEEB
```

## Input Mode Flows

### Interactive Mode Flow

```mermaid
flowchart TD
    Start([Start Interactive Mode]) --> Welcome[Display Welcome Message]
    Welcome --> SelectType[Prompt: Select Commit Type]
    
    SelectType --> TypeChosen{Type Selected?}
    TypeChosen -->|Cancel| Cancel
    TypeChosen -->|Selected| PromptScope[Prompt: Enter Scope]
    
    PromptScope --> ScopeEntered{Scope Entered?}
    ScopeEntered -->|Skip| PromptSubject
    ScopeEntered -->|Entered| ValidateScope{Valid Scope?}
    
    ValidateScope -->|Invalid| ShowScopeError[Show Error]
    ShowScopeError --> PromptScope
    ValidateScope -->|Valid| PromptSubject[Prompt: Enter Subject]
    
    PromptSubject --> SubjectEntered{Subject Entered?}
    SubjectEntered -->|Empty| ShowSubjectError[Error: Required]
    ShowSubjectError --> PromptSubject
    SubjectEntered -->|Entered| ValidateSubject{Valid Subject?}
    
    ValidateSubject -->|Too Long| WarnLength[Warn: Over 50 chars]
    ValidateSubject -->|Invalid Chars| ShowCharError[Show Error]
    ShowCharError --> PromptSubject
    WarnLength --> PromptSubject
    ValidateSubject -->|Valid| PromptBody[Prompt: Enter Body]
    
    PromptBody --> BodyEntered{Body Entered?}
    BodyEntered -->|Skip| PromptBreaking
    BodyEntered -->|Entered| PromptFooter[Prompt: Enter Footer]
    
    PromptFooter --> FooterEntered{Footer Entered?}
    FooterEntered -->|Skip| PromptBreaking
    FooterEntered -->|Entered| PromptBreaking[Prompt: Breaking Change?]
    
    PromptBreaking --> BreakingResponse{Breaking?}
    BreakingResponse -->|Yes| WarnBreaking[Show Breaking Warning]
    WarnBreaking --> BuildMessage
    BreakingResponse -->|No| BuildMessage[Build Commit Message]
    
    BuildMessage --> Return([Return Message])
    Cancel([Cancel]) --> Return
    
    style Start fill:#90EE90
    style Return fill:#87CEEB
    style Cancel fill:#FFB6C1
```

### Interactive Mode Sections

```mermaid
stateDiagram-v2
    [*] --> Header
    
    state Header {
        [*] --> SelectType
        SelectType --> EnterScope
        EnterScope --> EnterSubject
        EnterSubject --> [*]
    }
    
    Header --> Body
    
    state Body {
        [*] --> PromptBody
        PromptBody --> EnterBody
        EnterBody --> [*]
        PromptBody --> Skip: Empty
        Skip --> [*]
    }
    
    Body --> Footer
    
    state Footer {
        [*] --> PromptFooter
        PromptFooter --> EnterFooter
        EnterFooter --> [*]
        PromptFooter --> Skip: Empty
        Skip --> [*]
    }
    
    Footer --> BreakingCheck
    
    state BreakingCheck {
        [*] --> Prompt
        Prompt --> Yes: Breaking
        Prompt --> No: Not Breaking
        Yes --> [*]
        No --> [*]
    }
    
    BreakingCheck --> [*]
```

### Editor Mode Flow

```mermaid
flowchart TD
    Start([Start Editor Mode]) --> DetectEditor{Editor Configured?}
    
    DetectEditor -->|GIT_EDITOR| UseGitEditor[Use GIT_EDITOR]
    DetectEditor -->|VISUAL| UseVisual[Use VISUAL]
    DetectEditor -->|EDITOR| UseEditor[Use EDITOR]
    DetectEditor -->|None| UseFallback[Use Default: vi]
    
    UseGitEditor --> GenerateTemplate
    UseVisual --> GenerateTemplate
    UseEditor --> GenerateTemplate
    UseFallback --> GenerateTemplate
    
    GenerateTemplate[Generate Template] --> WriteTemp[Write to Temp File]
    WriteTemp --> OpenEditor[Open Editor]
    
    OpenEditor --> EditorClosed{Editor Closed?}
    
    EditorClosed -->|Cancelled| Cleanup
    EditorClosed -->|Saved| ReadFile[Read Temp File]
    
    ReadFile --> ParseContent[Parse Content]
    ParseContent --> RemoveComments[Remove Comment Lines]
    RemoveComments --> ExtractSections[Extract Sections]
    
    ExtractSections --> ValidateFormat{Valid Format?}
    
    ValidateFormat -->|Invalid| ShowParseError[Show Parse Errors]
    ShowParseError --> OfferRetry{Retry?}
    OfferRetry -->|Yes| OpenEditor
    OfferRetry -->|No| Cleanup
    
    ValidateFormat -->|Valid| BuildMessage[Build Commit Message]
    
    BuildMessage --> Cleanup[Cleanup Temp File]
    Cleanup --> Return([Return Message])
    
    style Start fill:#90EE90
    style Return fill:#87CEEB
```

### Editor Template Structure

```
# Type your commit message here
# Lines starting with '#' will be ignored
#
# Format: <type>(<scope>): <subject>
#
# Example: feat(api): add user authentication
#
# Commit types:
#   feat:     A new feature
#   fix:      A bug fix
#   docs:     Documentation changes
#   style:    Code style changes
#   refactor: Code refactoring
#   perf:     Performance improvements
#   test:     Test changes
#   build:    Build system changes
#   ci:       CI configuration changes
#   chore:    Other changes
#   revert:   Revert previous commit

# Subject line (required, max 50 chars):


# Body (optional, explain what and why):


# Footer (optional, reference issues):


# Breaking change? (yes/no):

```

### Direct Mode Flow

```mermaid
flowchart TD
    Start([Start Direct Mode]) --> ParseFlags[Parse CLI Flags]
    
    ParseFlags --> HasMessage{Has -m flag?}
    
    HasMessage -->|No| Error[Error: Message Required]
    Error --> End
    
    HasMessage -->|Yes| ExtractMessage[Extract Message Text]
    
    ExtractMessage --> HasType{Has -t flag?}
    
    HasType -->|Yes| UseProvidedType[Use Provided Type]
    HasType -->|No| ParseType[Parse Type from Message]
    
    UseProvidedType --> HasScope
    ParseType --> TypeValid{Valid Type?}
    
    TypeValid -->|No| ErrorType[Error: Invalid Type]
    ErrorType --> End
    TypeValid -->|Yes| HasScope{Has -s flag?}
    
    HasScope -->|Yes| UseProvidedScope[Use Provided Scope]
    HasScope -->|No| ParseScope[Parse Scope from Message]
    
    UseProvidedScope --> BuildMessage
    ParseScope --> BuildMessage[Build Commit Message]
    
    BuildMessage --> ValidateMessage{Valid?}
    
    ValidateMessage -->|Invalid| ShowErrors[Show Errors]
    ShowErrors --> End
    
    ValidateMessage -->|Valid| Return([Return Message])
    
    End([End with Error])
    
    style Start fill:#90EE90
    style Return fill:#87CEEB
    style End fill:#FFB6C1
```

## Git Operations Flow

### Staging Check Flow

```mermaid
flowchart TD
    Start([Check Staging]) --> OpenRepo[Open Repository]
    
    OpenRepo --> RepoFound{Repository Found?}
    
    RepoFound -->|No| ErrorNoRepo[Error: Not a Git Repo]
    ErrorNoRepo --> End
    
    RepoFound -->|Yes| GetStatus[Get Status]
    
    GetStatus --> HasStaged{Has Staged Files?}
    
    HasStaged -->|Yes| GetDiff[Get Staged Diff]
    GetDiff --> CountChanges[Count Changes]
    CountChanges --> ReturnStatus[Return Status]
    
    HasStaged -->|No| ReturnEmpty[Return: No Changes]
    
    ReturnStatus --> End([End])
    ReturnEmpty --> End
    
    style Start fill:#90EE90
    style End fill:#87CEEB
```

### Commit Execution Flow

```mermaid
flowchart TD
    Start([Execute Commit]) --> OpenRepo[Open Repository]
    
    OpenRepo --> GetSignature[Get User Signature]
    GetSignature --> SignatureValid{Valid Signature?}
    
    SignatureValid -->|No| ConfigError[Error: Configure Git User]
    ConfigError --> End
    
    SignatureValid -->|Yes| GetIndex[Get Index]
    GetIndex --> WriteTree[Write Tree]
    
    WriteTree --> TreeSuccess{Success?}
    TreeSuccess -->|No| TreeError[Error: Write Failed]
    TreeError --> End
    
    TreeSuccess -->|Yes| GetHead[Get HEAD Reference]
    
    GetHead --> HasParent{HEAD Exists?}
    
    HasParent -->|Yes| GetParentCommit[Get Parent Commit]
    GetParentCommit --> CreateCommit[Create Commit with Parent]
    
    HasParent -->|No| CreateInitial[Create Initial Commit]
    
    CreateCommit --> CommitSuccess{Success?}
    CreateInitial --> CommitSuccess
    
    CommitSuccess -->|No| CommitError[Error: Commit Failed]
    CommitError --> End
    
    CommitSuccess -->|Yes| UpdateRef[Update HEAD Reference]
    
    UpdateRef --> RefSuccess{Success?}
    
    RefSuccess -->|No| RefError[Error: Update Failed]
    RefError --> End
    
    RefSuccess -->|Yes| GetCommitId[Get Commit SHA]
    GetCommitId --> Return([Return Commit ID])
    
    End([End with Error])
    
    style Start fill:#90EE90
    style Return fill:#87CEEB
    style End fill:#FFB6C1
```

## Compilation Flow

### DSL Compilation Pipeline

```mermaid
flowchart LR
    Source[Template Source] --> Lexer[Lexer]
    
    Lexer --> TokenStream[Token Stream]
    
    TokenStream --> Parser[Parser]
    
    Parser --> AST[Abstract Syntax Tree]
    
    AST --> Validator[Semantic Validator]
    
    Validator --> Evaluator[Evaluator]
    
    Evaluator --> Output[Rendered Output]
    
    subgraph "Lexical Analysis"
        Lexer
        TokenStream
    end
    
    subgraph "Syntactic Analysis"
        Parser
        AST
    end
    
    subgraph "Semantic Analysis"
        Validator
    end
    
    subgraph "Code Generation"
        Evaluator
        Output
    end
    
    style Lexer fill:#FFE4B5
    style Parser fill:#FFE4B5
    style Validator fill:#FFE4B5
    style Evaluator fill:#FFE4B5
```

### Lexer State Machine

```mermaid
stateDiagram-v2
    [*] --> Text
    
    Text --> OpenDelim: {{
    Text --> Text: Regular char
    Text --> [*]: EOF
    
    OpenDelim --> Variable: identifier
    OpenDelim --> Control: %
    OpenDelim --> Comment: #
    
    Variable --> CloseDelim: }}
    CloseDelim --> Text
    
    Control --> ControlType: if/for/end
    ControlType --> CloseControl: %}
    CloseControl --> Text
    
    Comment --> CloseComment: #}
    CloseComment --> Text
```

### Parser Flow

```mermaid
flowchart TD
    Start([Start Parsing]) --> ReadToken[Read Next Token]
    
    ReadToken --> TokenType{Token Type?}
    
    TokenType -->|Text| CreateText[Create Text Node]
    TokenType -->|Variable| CreateVar[Create Variable Node]
    TokenType -->|Control| ParseControl{Control Type?}
    TokenType -->|EOF| Done
    
    ParseControl -->|If| ParseIf[Parse If Statement]
    ParseControl -->|For| ParseFor[Parse For Loop]
    ParseControl -->|End| ParseEnd[Parse End Tag]
    
    CreateText --> AddNode[Add to AST]
    CreateVar --> AddNode
    ParseIf --> AddNode
    ParseFor --> AddNode
    ParseEnd --> ValidateNesting{Valid Nesting?}
    
    ValidateNesting -->|No| Error[Syntax Error]
    ValidateNesting -->|Yes| AddNode
    
    AddNode --> ReadToken
    
    Done([Return AST])
    Error --> End([End with Error])
    
    style Start fill:#90EE90
    style Done fill:#87CEEB
    style Error fill:#FFB6C1
```

## State Machines

### Application State Machine

```mermaid
stateDiagram-v2
    [*] --> Initializing
    
    Initializing --> Ready: Config Loaded
    Initializing --> Error: Init Failed
    
    Ready --> CollectingInput: Start Input
    
    CollectingInput --> Validating: Input Complete
    CollectingInput --> Cancelled: User Cancel
    
    Validating --> Previewing: Valid
    Validating --> CollectingInput: Invalid
    
    Previewing --> Confirming: Show Preview
    
    Confirming --> Committing: Confirmed
    Confirming --> CollectingInput: Edit
    Confirming --> Cancelled: Rejected
    
    Committing --> Success: Commit OK
    Committing --> Error: Commit Failed
    
    Success --> [*]
    Error --> [*]
    Cancelled --> [*]
```

### Input Collection State Machine

```mermaid
stateDiagram-v2
    [*] --> Idle
    
    Idle --> PromptingType: Start
    
    PromptingType --> PromptingScope: Type Selected
    PromptingType --> Idle: Cancelled
    
    PromptingScope --> PromptingSubject: Scope Entered/Skipped
    PromptingScope --> PromptingType: Back
    
    PromptingSubject --> ValidatingSubject: Subject Entered
    PromptingSubject --> PromptingScope: Back
    
    ValidatingSubject --> PromptingSubject: Invalid
    ValidatingSubject --> PromptingBody: Valid
    
    PromptingBody --> PromptingFooter: Body Entered/Skipped
    PromptingBody --> PromptingSubject: Back
    
    PromptingFooter --> PromptingBreaking: Footer Entered/Skipped
    PromptingFooter --> PromptingBody: Back
    
    PromptingBreaking --> Complete: Response Received
    PromptingBreaking --> PromptingFooter: Back
    
    Complete --> [*]
```

## Error Handling Flow

### Error Propagation

```mermaid
flowchart TD
    Start[Error Occurs] --> ErrorType{Error Type?}
    
    ErrorType -->|Domain Error| DomainHandler[Domain Error Handler]
    ErrorType -->|Git Error| GitHandler[Git Error Handler]
    ErrorType -->|IO Error| IOHandler[IO Error Handler]
    ErrorType -->|Input Error| InputHandler[Input Error Handler]
    
    DomainHandler --> FormatError[Format Error Message]
    GitHandler --> TranslateError[Translate Git Error]
    IOHandler --> FormatError
    InputHandler --> FormatError
    
    TranslateError --> FormatError
    
    FormatError --> AddContext[Add Context]
    AddContext --> LogError{Log Error?}
    
    LogError -->|Debug Mode| WriteLog[Write to Log]
    LogError -->|Production| DisplayUser[Display to User]
    WriteLog --> DisplayUser
    
    DisplayUser --> Recoverable{Recoverable?}
    
    Recoverable -->|Yes| OfferRetry[Offer Retry]
    Recoverable -->|No| Exit[Exit Gracefully]
    
    OfferRetry --> UserChoice{Retry?}
    UserChoice -->|Yes| Return([Return to Previous State])
    UserChoice -->|No| Exit
    
    Exit --> Cleanup[Cleanup Resources]
    Cleanup --> End([End])
    
    style Start fill:#FFB6C1
    style Return fill:#87CEEB
    style End fill:#90EE90
```

### Error Recovery Strategy

```mermaid
graph TD
    Error[Error Detected] --> Classify{Classify Error}
    
    Classify -->|Fatal| LogFatal[Log Fatal Error]
    Classify -->|Recoverable| TryRecover[Attempt Recovery]
    Classify -->|User Error| ShowHelp[Show Help Message]
    
    LogFatal --> SafeExit[Safe Exit]
    
    TryRecover --> RecoverySuccess{Success?}
    RecoverySuccess -->|Yes| Continue[Continue Execution]
    RecoverySuccess -->|No| LogFatal
    
    ShowHelp --> OfferRetry[Offer to Retry]
    OfferRetry --> UserResponse{User Choice?}
    UserResponse -->|Retry| Continue
    UserResponse -->|Abort| SafeExit
    
    Continue --> Normal[Normal Execution]
    SafeExit --> Exit[Exit Program]
    
    style Error fill:#FFB6C1
    style Continue fill:#90EE90
    style Normal fill:#87CEEB
```

## Sequence Diagrams

### Complete Commit Sequence

```mermaid
sequenceDiagram
    actor User
    participant CLI
    participant App
    participant Input
    participant Domain
    participant Git
    
    User->>CLI: commando
    CLI->>App: Initialize
    App->>App: Load Config
    App->>Git: Check Repository
    Git-->>App: Repository OK
    
    App->>Input: Create Input Handler
    App->>Input: Collect Message
    
    Input->>User: Prompt Type
    User->>Input: Select "feat"
    
    Input->>User: Prompt Scope
    User->>Input: Enter "api"
    
    Input->>User: Prompt Subject
    User->>Input: Enter "add endpoint"
    
    Input->>User: Prompt Body
    User->>Input: Skip
    
    Input->>User: Prompt Footer
    User->>Input: Skip
    
    Input->>User: Prompt Breaking
    User->>Input: No
    
    Input->>Domain: Build Message
    Domain->>Domain: Validate
    Domain-->>Input: Valid Message
    Input-->>App: Return Message
    
    App->>Git: Check Staging
    Git-->>App: Has Changes
    
    App->>User: Preview Message
    User->>App: Confirm
    
    App->>Git: Execute Commit
    Git->>Git: Create Commit
    Git-->>App: Commit SHA
    
    App->>User: Display Success
```

---

For more details, see:
- [Architecture](ARCHITECTURE.md)
- [Requirements](REQUIREMENTS.md)
- [Development Guide](DEVELOPMENT.md)

