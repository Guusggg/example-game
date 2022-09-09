 
#[link(name = "client")]
extern "C" {
    #[link_name = "Z_my_gameZ_game_init"]
    pub static game_init: Option<unsafe extern "C" fn() -> u32>;
    #[link_name = "Z_my_gameZ_game_loop"]
    pub static game_loop: Option<unsafe extern "C" fn() -> ()>;    
    #[link_name = "Z_my_gameZ_game_get_score"]
    pub static game_get_score: Option<unsafe extern "C" fn() -> u32>;
}
