uv init                               # 初始化项目（生成 pyproject.toml）
uv init --lib                         # 初始化库项目
uv init --app                         # 初始化应用项目
uv venv --python 3.12                 # 用指定 Python 创建 .venv
uv python list                        # 查看可用/已安装 Python
uv python install 3.12                # 安装特定 Python（uv 管理 Python 版本时用）
uv python pin 3.12                    # 在项目写入 .python-version

uv add <pkg>                          # 添加依赖（写 pyproject + lock）
uv add --dev <pkg>                    # 添加开发依赖
uv add -r requirements.txt            # 从 requirements 文件导入依赖
uv remove <pkg>                       # 移除依赖
uv lock                               # 生成/更新锁文件
uv lock --check                       # 检查锁文件是否最新
uv sync                               # 按锁文件同步安装
uv sync --no-dev                      # 只安装生产依赖
uv sync --check                       # 检查环境是否与锁文件一致

uv run <cmd...>                       # 在项目环境中运行命令
uv run -m pytest                      # 运行测试模块
uv run --with ruff ruff check .       # 临时安装并运行工具（不写入依赖）
uv tree                               # 查看项目依赖树

uv export --format requirements.txt -o requirements.txt  # 导出 requirements.txt
uv pip compile requirements.in -o requirements.txt       # 锁定 requirements.in
uv pip sync requirements.txt                             # 按 requirements.txt 同步环境

uv tool install ruff                  # 安装全局 CLI 工具
uv tool run ruff check .              # 一次性运行工具（等价 uvx ruff check .）
uv tool list                          # 查看已安装的工具
