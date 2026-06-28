# Zed Flutter 扩展本地打包
#
# 用法：
#   make extension       — 构建并打包 WASM 扩展（开发）
#   make build           — 构建所有（WASM + Bridge Server）
#   make bridge          — 仅构建 Bridge Server
#   make extension-zip   — 构建并打包为 ZIP（用于安装到 Zed）
#   make clean           — 清理产物

EXTENSION_DIR = flutter_extension
BRIDGE_PKG = flutter-bridge-server
EXTENSION_PKG = flutter-extension
TARGET_DIR = target
WASM_TARGET = wasm32-wasip1
BUILD_DIR = build

# 检测操作系统
ifeq ($(OS),Windows_NT)
	DETECTED_OS := windows
	EXT = .exe
	ZIP = powershell -Command "Compress-Archive -Path '$(BUILD_DIR)/flutter_extension' -DestinationPath '$(BUILD_DIR)/flutter-zed-extension.zip' -Force"
else
	DETECTED_OS := linux
	EXT =
	ZIP = cd $(BUILD_DIR) && zip -r flutter-zed-extension.zip flutter_extension/
endif

.PHONY: all build bridge extension extension-zip clean

all: extension

# 构建 Bridge Server（原生二进制）
bridge:
	cargo build --package $(BRIDGE_PKG) --release

# 构建并打包 WASM 扩展
extension:
	cargo build --package $(EXTENSION_PKG) --target $(WASM_TARGET) --release
	@echo "=== 打包扩展 ==="
	rm -rf $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)/flutter_extension
	cp $(TARGET_DIR)/$(WASM_TARGET)/release/$(EXTENSION_PKG).wasm $(BUILD_DIR)/flutter_extension/
	cp $(EXTENSION_DIR)/extension.toml $(BUILD_DIR)/flutter_extension/
	cp -r $(EXTENSION_DIR)/languages $(BUILD_DIR)/flutter_extension/
	cp -r $(EXTENSION_DIR)/snippets $(BUILD_DIR)/flutter_extension/
	cp -r $(EXTENSION_DIR)/debug_adapter_schemas $(BUILD_DIR)/flutter_extension/
	@echo ""
	@echo "=== 打包完成 ==="
	@echo "扩展路径: $(BUILD_DIR)/flutter_extension/"
	@echo "在 Zed 中安装本地扩展时，请选择此目录。"
	@echo ""

# 构建所有组件
build: bridge extension

# 构建并打包为 ZIP
extension-zip: extension
	$(ZIP)
	@echo "ZIP 包: $(BUILD_DIR)/flutter-zed-extension.zip"

# 清理
clean:
	rm -rf $(BUILD_DIR)
	cargo clean
