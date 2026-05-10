use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code as KeyCode, HotKey, Modifiers as GlobalModifiers},
};
use std::thread;

pub fn generate_global_listener() {
    thread::spawn(move || {
        println!("✅ 后台全局按键监听已启动，按任意键打印...");
        let manager = GlobalHotKeyManager::new().unwrap();
        let combo_key = HotKey::new(
            Some(GlobalModifiers::CONTROL | GlobalModifiers::ALT), // 组合键修饰符
            KeyCode::KeyO,                                         // 主按键 H
        );
        // 循环捕获全局按键
        // for key in all_keys {
        //     let hotkey = HotKey::new(None, key);
        //     manager.register(hotkey).unwrap();
        // }
        manager.register(combo_key).unwrap();

        // 持续监听按键触发
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                println!("{:?}", event);
            }
        }
    });
}
