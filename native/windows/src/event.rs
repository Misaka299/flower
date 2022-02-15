pub enum Event {
    /// 重画
    OnPrint,
    /// 窗口移动
    WindowMove,
    /// 窗口大小被改变
    WindowResized,
    /// 窗口创建
    WindowCreate,
    /// 窗口关闭
    WindowClose,
    /// 鼠标进入
    MouseEnter,
    /// 鼠标离开
    MouseLeave,
    /// 鼠标移动
    MouseMove,
    /// 鼠标滚动
    MouseScroll,
    /// 鼠标左键按下
    LButtonDown(i32, i32),
    /// 鼠标右键按下
    RButtonDown,
    /// 鼠标中键按下
    MButtonDown,
    /// 鼠标左键按下
    LButtonUp,
    /// 鼠标右键按下
    RButtonUp,
    /// 鼠标中键按下
    MButtonUp,
}