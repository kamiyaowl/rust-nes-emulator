#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const uintptr_t EMBEDDED_EMULATOR_NUM_OF_COLOR = 3;

static const uintptr_t EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT = 240;

static const uintptr_t EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH = 256;

enum class KeyEvent : uint8_t {
  PressA,
  PressB,
  PressSelect,
  PressStart,
  PressUp,
  PressDown,
  PressLeft,
  PressRight,
  ReleaseA,
  ReleaseB,
  ReleaseSelect,
  ReleaseStart,
  ReleaseUp,
  ReleaseDown,
  ReleaseLeft,
  ReleaseRight,
};

extern "C" {

void EmbeddedEmulator_init();

/// .nesファイルを読み込みます
/// `data` - nesファイルのバイナリ
bool EmbeddedEmulator_load();

/// エミュレータをリセットします
/// カセットの中身はリセットしないので実機のリセット相当の処理です
void EmbeddedEmulator_reset();

/// キー入力します
void EmbeddedEmulator_update_key(KeyEvent key);

/// 描画領域1面分更新します
void EmbeddedEmulator_update_screen(uint8_t (*fb)[EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT][EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH][EMBEDDED_EMULATOR_NUM_OF_COLOR]);

void rust_eh_personality();

} // extern "C"
