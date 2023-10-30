trait Listener {
    fn update(&self);
    fn updateEditor(&mut self, data: String);
}
struct Editor {
    openFile: String,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            openFile: String::new(),
        }
    }
    fn setFile(&mut self, file: String) {
        self.openFile = file;
    }
}

struct EditorListener<'a> {
    editor: &'a mut Editor,
}

impl<'a> Listener for EditorListener<'a> {
    fn update(&self) {}
    fn updateEditor(&mut self, new_file: String) {
        self.editor.setFile(new_file);
    }
}
impl<'a> EditorListener<'a> {
    pub fn new(editor: &'a mut Editor) -> Self {
        Self { editor }
    }
}
struct TreeEventManager {
    listeners: Vec<Box<dyn Listener>>,
}
impl TreeEventManager {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    fn addListener(&mut self, listener: Box<dyn Listener>) {
        self.listeners.push(listener);
    }
    fn notify(&mut self, data: String) {
        for (_, elem) in &mut self.listeners.iter().enumerate() {
            elem.update();
        }
    }
}
struct Tree<'a> {
    files: Vec<String>,
    context: &'a mut TreeEventManager,
}
impl<'a> Tree<'a> {
    pub fn new(context: &'a mut TreeEventManager) -> Self {
        Self {
            files: Vec::new(),
            context,
        }
    }
    pub fn renderFiles(&mut self, files: Vec<String>) {
        let mut aux = files.clone();
        self.files.append(&mut aux);
    }
    pub fn open(&mut self, new_file: String) {
        self.context.notify(new_file);
    }
    pub fn getFiles(&self) {
        for elem in self.files.iter() {
            println!("File: {}", &elem);
        }
    }
}
#[test]
fn app() {
    let mut treeContext = TreeEventManager::new();
    let mut tree = Tree::new(&mut treeContext);
    let directory = vec![
        String::from("javascript.js"),
        String::from("node.js"),
        String::from("go.go"),
        String::from(".gitignore"),
    ];

    tree.renderFiles(directory);
    tree.getFiles();
    let mut editor = Editor::new();
    let mut editorListener = EditorListener::new(&mut editor);
    treeContext.addListener(Box::new(editorListener.updateEditor(String::new())));
}
