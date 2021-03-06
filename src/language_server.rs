use std::collections::HashMap;
use serde;
use serde::de::Error;
use serde_json::Value;

#[derive(Deserialize)]
pub struct CancelParams {
    /**
     * The request id to cancel.
     */
    pub id: String,
}

#[derive(Deserialize)]
pub struct DidChangeTextDocumentParams {
    /**
     * The document that did change. The version number points
     * to the version after all provided content changes have
     * been applied.
     */
    #[serde(rename="textDocument")]
    pub text_document: VersionedTextDocumentIdentifier,
    /**
     * The actual content changes.
     */
    #[serde(rename="contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

/// Text documents are identified using a URI. On the protocol level, URIs are passed as strings. The corresponding JSON structure looks like this:
#[derive(Deserialize)]
pub struct TextDocumentIdentifier {
    /**
     * The text document's URI.
     */
    pub uri: String,
}

/// An identifier to denote a specific version of a text document.
#[derive(Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    /**
     * The text document's URI.
     */
    pub uri: String,
    /**
     * The version number of this document.
     */
    pub version: u64,
}

#[derive(Deserialize)]
pub struct TextDocumentItem {
    /**
     * The text document's URI.
     */
    pub uri: String,

    /**
     * The text document's language identifier.
     */
    #[serde(rename="languageId")]
    pub language_id: String,

    /**
     * The version number of this document (it will strictly increase after each
     * change, including undo/redo).
     */
    pub version: u64,

    /**
     * The content of the opened text document.
     */
    pub text: String,
}

#[derive(Deserialize)]
pub struct DidOpenTextDocumentParams {
    /**
     * The document that was opened.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentItem,
}

/**
 * An event describing a change to a text document. If range and rangeLength are omitted
 * the new text is considered to be the full content of the document.
 */
#[derive(Deserialize)]
pub struct TextDocumentContentChangeEvent {
    /**
     * The range of the document that changed.
     */
    pub range: Option<Range>,
    /**
     * The length of the range that got replaced.
     */
    #[serde(rename="rangeLength")]
    pub range_length: Option<u64>,
    /**
     * The new text of the document.
     */
    pub text: String,
}

#[derive(Deserialize)]
pub struct DidCloseTextDocumentParams {
    /**
     * The document that was closed.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,
}

#[derive(Deserialize)]
pub struct DidSaveTextDocumentParams {
    /**
     * The document that was saved.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,
}

#[derive(Deserialize)]
pub struct DidChangeWatchedFilesParams {
    /**
     * The actual file events.
     */
    pub changes: Vec<FileEvent>,
}

/**
 * The file event type.
 */
pub enum FileChangeType {
    /**
     * The file got created.
     */
    Created = 1,
    /**
     * The file got changed.
     */
    Changed = 2,
    /**
     * The file got deleted.
     */
    Deleted = 3
}

impl serde::Deserialize for FileChangeType {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        Ok(match try!(u8::deserialize(deserializer)) {
            1 => FileChangeType::Created,
            2 => FileChangeType::Changed,
            3 => FileChangeType::Deleted,
            _ => return Err(D::Error::invalid_value("Expected a value of 1, 2 or 3 to deserialze to FileChangeType")),
        })
    }
}

/**
 * An event describing a file change.
 */
#[derive(Deserialize)]
pub struct FileEvent {
    /**
     * The file's URI.
     */
    pub uri: String,
    /**
     * The change type.
     */
    pub typ: FileChangeType,
}

/// Position in a text document expressed as zero-based line and character offset.
#[derive(Copy, Clone, Default, Deserialize, Serialize)]
pub struct Position {
    /// Line position in a document (zero-based).
    pub line: u64,
    /// Character offset on a line in a document (zero-based).
    pub character: u64,
}

#[derive(Copy, Clone, Default, Deserialize, Serialize)]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

/// Represents a location inside a resource, such as a line inside a text file.
pub struct Location {
    pub uri: String,
    pub range: Range,
}



/// A parameter literal used in requests to pass a text document and a position inside that document.
#[derive(Deserialize)]
pub struct TextDocumentPositionParams {
    /**
     * The text document.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,
    /**
     * The position inside the text document.
     */
    pub position: Position,
}

#[derive(Deserialize)]
pub struct InitializeParams {
    /**
     * The process Id of the parent process that started
     * the server.
     */
    #[serde(rename="processId")]
    pub process_id: u64,

    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     */
    #[serde(rename="rootPath")]
    pub root_path: Option<String>,

    /**
     * The capabilities provided by the client (editor)
     */
    pub capabilities: ClientCapabilities,
}

#[derive(Deserialize)]
pub struct ClientCapabilities {
    _dummy: Option<()>,
}

#[derive(Default, Serialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
}

#[derive(Default, Serialize)]
pub struct InitializeError {
    /**
     * Indicates whether the client should retry to send the
     * initilize request after showing the message provided
     * in the ResponseError.
     */
    pub retry: bool,
}

#[derive(Default, Serialize)]
pub struct ServerCapabilities {
    /**
     * Defines how text documents are synced.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="textDocumentSync")]
    pub text_document_sync: Option<TextDocumentSyncKind>,
    /**
     * The server provides hover support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="hoverProvider")]
    pub hover_provider: Option<bool>,
    /**
     * The server provides completion support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="completionProvider")]
    pub completion_provider: Option<CompletionOptions>,
    /**
     * The server provides signature help support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="signatureHelpProvider")]
    pub signature_help_provider: Option<SignatureHelpOptions>,
    /**
     * The server provides goto definition support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="definitionProvider")]
    pub definition_provider: Option<bool>,
    /**
     * The server provides find references support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="referencesProvider")]
    pub references_provider: Option<bool>,
    /**
     * The server provides document highlight support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="documentHighlightProvider")]
    pub document_highlight_provider: Option<bool>,
    /**
     * The server provides document symbol support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="documentSymbolProvider")]
    pub document_symbol_provider: Option<bool>,
    /**
     * The server provides workspace symbol support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="workspaceSymbolProvider")]
    pub workspace_symbol_provider: Option<bool>,
    /**
     * The server provides code actions.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="codeActionProvider")]
    pub code_action_provider: Option<bool>,
    /**
     * The server provides code lens.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="codeLensProvider")]
    pub code_lens_provider: Option<CodeLensOptions>,
    /**
     * The server provides document formatting.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="documentFormattingProvider")]
    pub document_formatting_provider: Option<bool>,
    /**
     * The server provides document range formatting.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="documentRangeFormattingProvider")]
    pub document_range_formatting_provider: Option<bool>,
    /**
     * The server provides document formatting on typing.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="documentOnTypeFormattingProvider")]
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    /**
     * The server provides rename support.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="renameProvider")]
    pub rename_provider: Option<bool>,
}

/**
 * Defines how the host (editor) should sync document changes to the language server.
 */
#[derive(Clone, Copy)]
pub enum TextDocumentSyncKind {
    /**
     * Documents should not be synced at all.
     */
    None = 0,
    /**
     * Documents are synced by always sending the full content of the document.
     */
    Full = 1,
    /**
     * Documents are synced by sending the full content on open. After that only
     * incremental updates to the document are sent.
     */
    Incremental = 2,
}

impl serde::Serialize for TextDocumentSyncKind {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

/**
 * Completion options.
 */
#[derive(Default, Serialize)]
pub struct CompletionOptions {
    /**
     * The server provides support to resolve additional information for a completion item.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="resolveProvider")]
    pub resolve_provider: Option<bool>,

    /**
     * The characters that trigger completion automatically.
     */
    #[serde(rename="triggerCharacters")]
    pub trigger_characters: Vec<String>,
}

/**
 * Signature help options.
 */
#[derive(Serialize)]
pub struct SignatureHelpOptions {
    /**
     * The characters that trigger signature help automatically.
     */
    #[serde(skip_serializing_if="Vec::is_empty")]
    #[serde(rename="triggerCharacters")]
    pub trigger_characters: Vec<String>,
}

/**
 * Code Lens options.
 */
#[derive(Serialize)]
pub struct CodeLensOptions {
    /**
     * Code lens has a resolve provider as well.
     */
    #[serde(rename="resolveProvider")]
    pub resolve_provider: Option<bool>,
}

/**
 * Format document on type options
 */
#[derive(Serialize)]
pub struct DocumentOnTypeFormattingOptions {
    /**
     * A character on which formatting should be triggered, like `}`.
     */
    #[serde(rename="firstTriggerCharacter")]
    pub first_trigger_character: String,
    /**
     * More trigger characters.
     */
    #[serde(skip_serializing_if="Vec::is_empty")]
    #[serde(rename="moreTriggerCharacter")]
    pub more_trigger_character: Vec<String>,
}

/// A textual edit applicable to a text document.
#[derive(Default, Serialize)]
pub struct TextEdit {
    /**
     * The range of the text document to be manipulated. To insert
     * text into a document create a range where start === end.
     */
    pub range: Range,
    /**
     * The string to be inserted. For delete operations use an
     * empty string.
     */
    #[serde(rename="newText")]
    pub new_text: String,
}

/// A workspace edit represents changes to many resources managed in the workspace.
#[derive(Default, Serialize)]
pub struct WorkspaceEdit {
    /**
     * Holds changes to existing resources.
     */
    pub changes: HashMap<String, Vec<TextEdit>>,
}

/**
 * Represents a collection of [completion items](#CompletionItem) to be presented
 * in the editor.
 */
#[derive(Default, Serialize)]
pub struct CompletionList {
    /**
     * This list it not complete. Further typing should result in recomputing
     * this list.
     */
    #[serde(rename="isIncomplete")]
    pub is_incomplete: bool,
    /**
     * The completion items.
     */
    pub items: Vec<CompletionItem>,
}

#[derive(Default, Serialize)]
pub struct CompletionItem {
    /**
     * The label of this completion item. By default
     * also the text that is inserted when selecting
     * this completion.
     */
    pub label: String,
    /**
     * The kind of this completion item. Based of the kind
     * an icon is chosen by the editor.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    pub kind: Option<CompletionItemKind>,
    /**
     * A human-readable string with additional information
     * about this item, like type or symbol information.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,
    /**
     * A human-readable string that represents a doc-comment.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation: Option<String>,
    /**
     * A string that shoud be used when comparing this item
     * with other items. When `falsy` the label is used.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="sortText")]
    pub sort_text: Option<String>,
    /**
     * A string that should be used when filtering a set of
     * completion items. When `falsy` the label is used.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="filterText")]
    pub filter_text: Option<String>,
    /**
     * A string that should be inserted a document when selecting
     * this completion. When `falsy` the label is used.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="insertText")]
    pub insert_text: Option<String>,
    /**
     * An edit which is applied to a document when selecting
     * this completion. When an edit is provided the value of
     * insertText is ignored.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="textEdit")]
    pub text_edit: Option<TextEdit>,
    /**
     * An data entry field that is preserved on a completion item between
     * a completion and a completion resolve request.
     */
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<Value>,
}

/**
 * The kind of a completion entry.
 */
#[derive(Clone, Copy)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
}

impl serde::Serialize for CompletionItemKind {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

/**
 * The result of a hove request.
 */
#[derive(Serialize)]
pub struct Hover {
    /**
     * The hover's content
     */
    pub contents: Vec<MarkedString>,

    /**
     * An optional range
     */
    pub range: Option<Range>,
}

pub enum MarkedString {
    String(String),
    LanguageString {
        language: String,
        value: String,
    },
}

impl serde::Serialize for MarkedString {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            MarkedString::String(ref s) => serializer.serialize_str(s),
            MarkedString::LanguageString { ref language, ref value } => {
                #[derive(Serialize)]
                struct Variant<'s> {
                    language: &'s str,
                    value: &'s str,
                }
                Variant {
                        language: language,
                        value: value,
                    }
                    .serialize(serializer)
            }
        }
    }
}

/**
 * Signature help represents the signature of something
 * callable. There can be multiple signature but only one
 * active and only one active parameter.
 */
#[derive(Serialize)]
pub struct SignatureHelp {
    /**
     * One or more signatures.
     */
    pub signatures: Vec<SignatureInformation>,

    /**
     * The active signature.
     */
    #[serde(rename="activeSignature")]
    pub active_signature: Option<u64>,

    /**
     * The active parameter of the active signature.
     */
    #[serde(rename="activeParameter")]
    pub active_parameter: Option<u64>,
}

/**
 * Represents the signature of something callable. A signature
 * can have a label, like a function-name, a doc-comment, and
 * a set of parameters.
 */
#[derive(Serialize)]
pub struct SignatureInformation {
    /**
     * The label of this signature. Will be shown in
     * the UI.
     */
    pub label: String,

    /**
     * The human-readable doc-comment of this signature. Will be shown
     * in the UI but can be omitted.
     */
    pub documentation: String,

    /**
     * The parameters of this signature.
     */
    #[serde(skip_serializing_if="Vec::is_empty")]
    pub parameters: Vec<ParameterInformation>,
}

/**
 * Represents a parameter of a callable-signature. A parameter can
 * have a label and a doc-comment.
 */
#[derive(Serialize)]
pub struct ParameterInformation {
    /**
     * The label of this signature. Will be shown in
     * the UI.
     */
    pub label: String,

    /**
     * The human-readable doc-comment of this signature. Will be shown
     * in the UI but can be omitted.
     */
    #[serde(skip_serializing_if="String::is_empty")]
    pub documentation: String,
}

#[derive(Deserialize)]
pub struct ReferenceParams {
    /**
     * The text document.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,
    /**
     * The position inside the text document.
     */
    pub position: Position,

    pub context: ReferenceContext,
}

#[derive(Deserialize)]
pub struct ReferenceContext {
    /**
     * Include the declaration of the current symbol.
     */
    #[serde(rename="includeDeclaration")]
    pub include_declaration: bool,
}

/**
 * A document highlight is a range inside a text document which deserves
 * special attention. Usually a document highlight is visualized by changing
 * the background color of its range.
 */
#[derive(Serialize)]
pub struct DocumentHighlight {
    /**
     * The range this highlight applies to.
     */
    pub range: Range,

    /**
     * The highlight kind, default is DocumentHighlightKind.Text.
     */
    pub kind: Option<DocumentHighlightKind>,
}

/**
 * A document highlight kind.
 */
#[derive(Copy, Clone)]
pub enum DocumentHighlightKind {
    /**
     * A textual occurrance.
     */
    Text = 1,

    /**
     * Read-access of a symbol, like reading a variable.
     */
    Read = 2,

    /**
     * Write-access of a symbol, like writing to a variable.
     */
    Write = 3
}

impl serde::Serialize for DocumentHighlightKind {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Deserialize)]
pub struct DocumentSymbolParams {
    /**
     * The text document.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,
}

/**
 * Represents information about programming constructs like variables, classes,
 * interfaces etc.
 */
pub struct SymbolInformation {
    /**
     * The name of this symbol.
     */
    pub name: String,

    /**
     * The kind of this symbol.
     */
    pub kind: SymbolKind,

    /**
     * The location of this symbol.
     */
    pub location: Location,

    /**
     * The name of the symbol containing this symbol.
     */
    #[serde(rename="containerName")]
    pub container_name: String,
}

/**
 * A symbol kind.
 */
#[derive(Copy, Clone)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
}

impl serde::Serialize for SymbolKind {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

/**
 * The parameters of a Workspace Symbol Request.
 */
#[derive(Deserialize)]
pub struct WorkspaceSymbolParams {
    /**
     * A non-empty query string
     */
    pub query: String,
}

/**
 * Params for the CodeActionRequest
 */
#[derive(Deserialize)]
pub struct CodeActionParams {
    /**
     * The document in which the command was invoked.
     */
    pub text_document: TextDocumentIdentifier,

    /**
     * The range for which the command was invoked.
     */
    pub range: Range,

    /**
     * Context carrying additional information.
     */
    pub context: CodeActionContext,
}

/**
 * Contains additional diagnostic information about the context in which
 * a code action is run.
 */
#[derive(Deserialize)]
pub struct CodeActionContext {
    /**
     * An array of diagnostics.
     */
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Deserialize)]
pub struct CodeLensParams {
    /**
     * The document to request code lens for.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,
}

/**
 * A code lens represents a command that should be shown along with
 * source text, like the number of references, a way to run tests, etc.
 *
 * A code lens is _unresolved_ when no command is associated to it. For performance
 * reasons the creation of a code lens and resolving should be done in two stages.
 */
#[derive(Serialize)]
pub struct CodeLens {
    /**
     * The range in which this code lens is valid. Should only span a single line.
     */
    pub range: Range,

    /**
     * The command this code lens represents.
     */
    pub command: Option<Command>,

    /**
     * A data entry field that is preserved on a code lens item between
     * a code lens and a code lens resolve request.
     */
    pub data: Option<Value>,
}

#[derive(Deserialize)]
pub struct RenameParams {
    /**
     * The document to format.
     */
    #[serde(rename="textDocument")]
    pub text_document: TextDocumentIdentifier,

    /**
     * The position at which this request was sent.
     */
    pub position: Position,

    /**
     * The new name of the symbol. If the given name is not valid the
     * request must return a [ResponseError](#ResponseError) with an
     * appropriate message set.
     */
    #[serde(rename="newName")]
    pub new_name: String,
}

#[derive(Serialize)]
pub struct ShowMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    #[serde(rename="type")]
    pub typ: MessageType,

    /**
     * The actual message
     */
    pub message: String,
}

#[derive(Serialize)]
pub struct ShowMessageRequestParams {
    /**
     * The message type. See {@link MessageType}
     */
    #[serde(rename="type")]
    pub typ: MessageType,

    /**
     * The actual message
     */
    pub message: String,

    /**
     * The message action items to present.
     */
    #[serde(skip_serializing_if="Vec::is_empty")]
    pub actions: Vec<MessageActionItem>,
}

#[derive(Serialize)]
pub struct MessageActionItem {
    /**
     * A short title like 'Retry', 'Open Log' etc.
     */
    pub title: String,
}

#[derive(Serialize)]
pub struct LogMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    #[serde(rename="type")]
    pub typ: MessageType,

    /**
     * The actual message
     */
    pub message: String,
}

#[derive(Serialize)]
pub struct DidChangeConfigurationParams {
    /**
     * The actual changed settings
     */
    pub settings: Value,
}

#[derive(Clone, Copy)]
pub enum MessageType {
    /**
     * An error message.
     */
    Error = 1,
    /**
     * A warning message.
     */
    Warning = 2,
    /**
     * An information message.
     */
    Info = 3,
    /**
     * A log message.
     */
    Log = 4,
}

impl serde::Serialize for MessageType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Default, Serialize)]
pub struct PublishDiagnosticsParams {
    /**
     * The URI for which diagnostic information is reported.
     */
    pub uri: String,

    /**
     * An array of diagnostic information items.
     */
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Diagnostic {
    /**
     * The range at which the message applies
     */
    pub range: Range,

    /**
     * The diagnostic's severity. Can be omitted. If omitted it is up to the
     * client to interpret diagnostics as error, warning, info or hint.
     */
    pub severity: Option<DiagnosticSeverity>,

    /**
     * The diagnostic's code. Can be omitted.
     */
    pub code: String, // number | string;

    /**
     * A human-readable string describing the source of this
     * diagnostic, e.g. 'typescript' or 'super lint'.
     */
    pub source: Option<String>,

    /**
     * The diagnostic's message.
     */
    pub message: String,
}

#[derive(Clone, Copy)]
pub enum DiagnosticSeverity {
    /**
     * Reports an error.
     */
    Error = 1,
    /**
     * Reports a warning.
     */
    Warning = 2,
    /**
     * Reports an information.
     */
    Information = 3,
    /**
     * Reports a hint.
     */
    Hint = 4,
}

impl serde::Deserialize for DiagnosticSeverity {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        Ok(match try!(u8::deserialize(deserializer)) {
            1 => DiagnosticSeverity::Error,
            2 => DiagnosticSeverity::Warning,
            3 => DiagnosticSeverity::Information,
            4 => DiagnosticSeverity::Hint,
            _ => return Err(D::Error::invalid_value("Expected a value of 1, 2, 3 or 4 to deserialze to DiagnosticSeverity")),
        })
    }
}

impl serde::Serialize for DiagnosticSeverity {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

/// Represents a reference to a command. Provides a title which will be used to represent a command in the UI and, optionally, an array of arguments which will be passed to the command handler function when invoked.
#[derive(Default, Serialize)]
pub struct Command {
    /**
     * Title of the command, like `save`.
     */
    pub title: String,
    /**
     * The identifier of the actual command handler.
     */
    pub command: String,
    /**
     * Arguments that the command handler should be
     * invoked with.
     */
    #[serde(skip_serializing_if="Vec::is_empty")]
    pub arguments: Vec<Value>,
}
