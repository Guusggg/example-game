/* Automatically generated by wasm2c */
#ifndef MY_GAME_H_GENERATED_
#define MY_GAME_H_GENERATED_

#include <stdint.h>

#include "wasm-rt.h"

/* TODO(binji): only use stdint.h types in header */
#ifndef WASM_RT_CORE_TYPES_DEFINED
#define WASM_RT_CORE_TYPES_DEFINED
typedef uint8_t u8;
typedef int8_t s8;
typedef uint16_t u16;
typedef int16_t s16;
typedef uint32_t u32;
typedef int32_t s32;
typedef uint64_t u64;
typedef int64_t s64;
typedef float f32;
typedef double f64;
#endif

#ifdef __cplusplus
extern "C" {
#endif


/* import: 'env' 'console_log' */
extern void (*Z_envZ_console_log)(u32);
/* import: 'env' 'clear_screen' */
extern void (*Z_envZ_clear_screen)(void);
/* import: 'env' 'glActiveTexture' */
extern void (*Z_envZ_glActiveTexture)(u32);
/* import: 'env' 'draw_rect' */
extern void (*Z_envZ_draw_rect)(u32, u32, u32, u32);
void Z_my_game_init(void);
void Z_my_game_free(void);

/* export: 'memory' */
extern wasm_rt_memory_t (*Z_my_gameZ_memory);
/* export: '__wasm_call_ctors' */
extern void (*Z_my_gameZ___wasm_call_ctors)(void);
/* export: 'game_init' */
extern u32 (*Z_my_gameZ_game_init)(void);
/* export: 'game_loop' */
extern void (*Z_my_gameZ_game_loop)(void);
/* export: 'game_get_score' */
extern u32 (*Z_my_gameZ_game_get_score)(void);
/* export: '__indirect_function_table' */
extern wasm_rt_table_t (*Z_my_gameZ___indirect_function_table);
/* export: '__errno_location' */
extern u32 (*Z_my_gameZ___errno_location)(void);
/* export: 'stackSave' */
extern u32 (*Z_my_gameZ_stackSave)(void);
/* export: 'stackRestore' */
extern void (*Z_my_gameZ_stackRestore)(u32);
/* export: 'stackAlloc' */
extern u32 (*Z_my_gameZ_stackAlloc)(u32);

#ifdef __cplusplus
}
#endif

#endif  /* MY_GAME_H_GENERATED_ */
