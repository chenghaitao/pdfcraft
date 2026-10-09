<p align="center">
  <a href="https://getartcraft.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/artcraft-logo-white.svg">
      <img alt="ArtCraft" src="docs/brand/artcraft-logo.svg" width="200">
    </picture>
  </a>
</p>


<h1 align="center">PdfCraft</h1>

<p align="center">
  <b>PDF 工作台；Adobe Acrobat 的开源、净室重写版，用纯 Rust 打造。</b><br>
  在快速的原生应用中阅读、整理、合并、拆分和保护 PDF，全部用 Rust 从零写成。<br>
  macOS · Windows · Linux · FreeBSD · 网页版
</p>

<p align="center">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-12a58a">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-0a7563">
  <img alt="Platforms: macOS, Windows, Linux, FreeBSD, web" src="https://img.shields.io/badge/runs%20on-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20FreeBSD%20%C2%B7%20web-12a58a">
  <img alt="No account, no telemetry" src="https://img.shields.io/badge/no%20account-no%20telemetry-0a7563">
</p>

<p align="center">
  <img alt="简体中文" src="https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-%E5%BD%93%E5%89%8D-12a58a">
  <a href="README.md"><img alt="English" src="https://img.shields.io/badge/English-README-0a7563"></a>
</p>

<p align="center">
  <a href="https://discord.gg/artcraft"><img alt="加入 ArtCraft 的 Discord 社区" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://getartcraft.com/apps/pdfcraft"><b>getartcraft.com 上的 PdfCraft</b></a> ·
  <a href="https://getartcraft.com/">ArtCraft</a> ·
  <a href="https://getartcraft.com/apps">全部 Crafting 应用</a>
</p>

<br>

<p align="center">
  <img src="docs/images/pdfcraft-viewer.png" alt="PdfCraft 打开了 PdfCraft Showcase 封面页，左侧是全部工具面板，右侧是 20 条串接的批注" width="100%">
  <br>
  <sub>PdfCraft Showcase —— 一份 13 页的样例 PDF，已打开全部工具面板与串接批注。</sub>
</p>

> [!NOTE]
> **ArtCraft 是一个由各行各业创作者组成的社区。** 数字艺术、生成艺术、音乐、游戏
> &mdash; 只要你动手创造，就是其中一员。**[来 Discord 打个招呼](https://discord.gg/artcraft)。**

<p align="center">
  <a href="#亮点">亮点</a> ·
  <a href="#阅读一切皆可赏心悦目">阅读</a> ·
  <a href="#查找选中复制">查找</a> ·
  <a href="#像摆弄桌上的卡片一样整理页面">整理</a> ·
  <a href="#合并与拆分不丢任何东西">合并与拆分</a> ·
  <a href="#打开受保护文档并尊重其规则">保护</a> ·
  <a href="#批注表单图层与附件">表单与图层</a> ·
  <a href="#到处都能运行数据始终属于你">全平台</a> ·
  <a href="#也为智能体而建">智能体</a> ·
  <a href="#技术架构">技术架构</a> ·
  <a href="#快速开始">快速开始</a> ·
  <a href="#下一步">下一步</a> ·
  <a href="#下载">下载</a> ·
  <a href="#crafting-应用家族">Crafting 应用</a>
</p>

<p align="center">
  <sub>本文档为 <a href="README.md">README.md</a> 的简体中文版；如与英文版有出入，以英文版为准。</sub>
</p>

---

## 社区

PdfCraft 是 [ArtCraft](https://getartcraft.com) 的一部分。欢迎来打招呼、寻求帮助并关注开发进展：

- **Discord：[discord.gg/artcraft](https://discord.gg/artcraft)**。这是获取帮助和反馈意见最快的途径。应用标题栏里也有 Discord 按钮。
- **官网：**[getartcraft.com/apps/pdfcraft](https://getartcraft.com/apps/pdfcraft)
- **源码：**[github.com/storytold/pdfcraft](https://github.com/storytold/pdfcraft)

`docs/brand/` 中的 ArtCraft 名称与徽标是 ArtCraft 团队的商标，不属于开源范畴。它们只能原样使用，且只能作为 PdfCraft 的一部分使用（见 `docs/brand/LICENSE-brand.txt`）。分支和修改版必须移除它们。

## 亮点

<table>
<tr>
<td width="33%" valign="top">

### 忠实还原
真实世界的排版：各国文字、日文竖排、彩色 emoji、渐变、软遮罩与透明度。一切都按作者原本的意图渲染。

</td>
<td width="33%" valign="top">

### 无需担心
每次保存都在文件末尾追加你的改动，原始字节分毫不动。写入是原子的，撤销可以回溯得很深，误关闭也不会丢失内容。

</td>
<td width="33%" valign="top">

### 属于你自己
没有账号、没有遥测、不依赖云。离线可用，打开即用。引擎、命令行工具和应用全部开源。

</td>
</tr>
</table>

---

## 阅读一切，皆可赏心悦目

PdfCraft 渲染 PDF 时会照顾到那些让页面"看起来对"的细节：字距与连字、从右到左和复杂文字系统、CJK 竖排、彩色 emoji、底纹、混合模式、软遮罩和可选内容。

<p align="center">
  <img src="docs/images/pdfcraft-scripts.png" alt="《世界文字》页面：阿拉伯文、希伯来文、天城文、泰文、希腊文、西里尔文、中文、韩文、国际音标、亚美尼亚文、格鲁吉亚文和泰米尔文样例，右侧边栏是日文竖排" width="100%">
  <br>
  <sub>一页之内 12 种书写系统，另加日文竖排，缩放 125%。</sub>
</p>

- **深度缩放依旧锐利。** 大页面按块渲染，任何放大倍率下文字都保持清晰。
- **为糟糕的文件而生。** 每一页独立渲染，受损文档会被修复。在 983 个文件的 pdf.js 测试语料上，结果是 0 次崩溃。
- **适配各种任务的版式：** 连续滚动、单页、双页、视图旋转、全屏，以及不受打扰的阅读模式。
- **浅色与深色主题**，都为长时间阅读做了护眼设计。

<table>
<tr>
<td width="50%"><img src="docs/images/pdfcraft-twoup.png" alt="深色主题下的双页阅读模式：前言与《设置文字》章节，带首字下沉和突出引文"></td>
<td width="50%"><img src="docs/images/pdfcraft-dark.png" alt="深色主题下的代码与图像章节，一段语法着色清单和一张分形图像，批注面板已打开"></td>
</tr>
<tr>
<td align="center"><sub>双页阅读模式，适合长时间阅读</sub></td>
<td align="center"><sub>深色主题，批注面板已打开</sub></td>
</tr>
</table>

## 查找、选中、复制

边输入边搜索整个文档，用 <kbd>⌘G</kbd> 逐个跳转匹配项，选中的文字会按正确的阅读顺序输出。分栏、从右到左的文字，以及 CJK 都适用。

## 在长文档中导航

书签、页面缩略图，以及文档自带的自定义页码标签（i、ii、1、2……），让你在长文档中始终不迷失方向。

<table>
<tr>
<td width="50%"><img src="docs/images/pdfcraft-find.png" alt="查找栏显示 “type” 一词的第 10/16 个匹配项，在《表现力排版》章节标题中高亮"></td>
<td width="50%"><img src="docs/images/pdfcraft-bookmarks.png" alt="书签面板显示样例文档的嵌套大纲，页码标签如 Cover、i、ii，旁边是《世界文字》页面"></td>
</tr>
<tr>
<td align="center"><sub>边输入边查找：第 10/16 个匹配项</sub></td>
<td align="center"><sub>嵌套书签，配合文档自带页码标签</sub></td>
</tr>
</table>

---

## 像摆弄桌上的卡片一样整理页面

打开 **整理页面** 一眼看到全部页面：
- **选中页面：** 单击、<kbd>⌘</kbd>-单击或 <kbd>⇧</kbd>-单击。
- **改动它们：** 旋转、删除、插入空白页、插入其他文件的页面，以及向前或向后移动。
- **一切都能撤销：** <kbd>⌘Z</kbd>，然后保存。

<p align="center">
  <img src="docs/images/pdfcraft-organize.png" alt="整理页面网格，样例文档的页面以缩略图呈现，其中三页被选中，上方是页面工具栏" width="100%">
  <br>
  <sub>整理页面：三页已选中，页面工具在上方工具栏中。</sub>
</p>

<table>
<tr>
<td width="50%" valign="top">

**经得起推敲的撤销。** 每次改动都是历史中的一步，可以随意前后回溯。编辑菜单会写出这一步的名字（“撤销 旋转页面”），而且保存之后撤销依然有效。

**值得信赖的保存：**
- *增量：* 原始字节逐字节保持原样。
- *原子：* 先写入临时副本，再整体替换。
- *可验证：* 用 qpdf 独立校验。

未保存的文档，其标签页上会有一个圆点；关闭或退出前会先询问，避免内容丢失。改动每分钟自动保存一次。如果 PdfCraft 意外退出，下次打开时会询问是否恢复你的工作。加密文档在磁盘上保持加密。

</td>
<td width="50%" valign="top"><img src="docs/images/pdfcraft-split.png" alt="整理视图之上的拆分文档对话框，设为每页一个文件，并提示将由 13 页生成 13 个文件"><br><sub>拆分文档：每页一个文件，13 页生成 13 个文件。</sub></td>
</tr>
</table>

## 合并与拆分，不丢任何东西

**合并文件** 可以把任意数量的 PDF 并成一个。每个文件得到一个书签，它自己的书签嵌套在其下。

**提取** 把你选中的页面复制成新文档。**拆分** 每 *n* 页拆一次，或者在你指定的页面之前拆分。

过程中不会有任何东西悄悄消失：
- 链接和命名目标会被重新指向复制后的页面；
- 表单域保持可交互；
- 图层保留其默认显示/隐藏状态；
- 附件一并带走。

合并后文档的每一页都与来源逐像素一致。

```sh
pdfcraft-cli combine report.pdf appendix.pdf --out combined.pdf
pdfcraft-cli extract report.pdf --pages 1,3,5 --out highlights.pdf
pdfcraft-cli split   report.pdf --every 10 --out-dir parts/
```

---

## 打开受保护文档，并尊重其规则

PdfCraft 完整实现了 PDF 标准安全处理器：
- 从 40 位 RC4 到 AES-256 的每一个版本；
- 用户密码与所有者密码，包括用 SASLprep 归一化的 Unicode 密码；
- 加密过滤器，以及仅加密附件。

作者设了限制的文档会给出明确提示，PdfCraft 遵守其权限设置。输入所有者密码后限制即解除。对加密文档的编辑会以加密形式保存，沿用同一套密钥。

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/pdfcraft-properties.png" alt="文档属性对话框的说明选项卡，标题、作者、主题和关键词可编辑，另有安全性、字体和高级选项卡"><br><sub>文档属性，说明选项卡</sub></td>
<td width="50%" valign="top">

**文档属性** 显示：
- 文档的标题、作者、主题和关键词，均可编辑；
- 它用到的字体，以及每一种是否已嵌入；
- PDF 版本、页面尺寸、标签、表单域、图层和附件；
- 完整的安全信息：加密方式、由哪个密码打开，以及每一项权限。

</td>
</tr>
</table>

## 批注、表单、图层与附件

<table>
<tr>
<td width="50%"><img src="docs/images/pdfcraft-forms.png" alt="交互式表单页面，高亮显示文本域、复选框、单选按钮、一个列表和一个签名域，表单域面板列出全部 13 个域及其值"></td>
<td width="50%"><img src="docs/images/pdfcraft-layers.png" alt="审阅与标记页面，DRAFT 水印之下的高亮、图形、墨迹和 APPROVED 图章，图层面板中有 Draft watermark 与 Print-only notes"></td>
</tr>
<tr>
<td valign="top"><b>表单</b>：每个域及其实时值、域高亮，以及复选框、单选按钮、列表和签名，全部按作者设计的样子绘制。</td>
<td valign="top"><b>图层</b>：开关可选内容，页面立即重新渲染。<b>批注</b> 以串接对话的形式呈现，<b>附件</b> 可以打开或另存。</td>
</tr>
</table>

## 每一个工具，一次按键就能到手

按 <kbd>⌘K</kbd> 搜索每一个工具和命令，或者浏览 **全部工具** 目录。仍在开发中的工具会标注将要发布它的里程碑。

<table>
<tr>
<td width="50%"><img src="docs/images/pdfcraft-palette.png" alt="命令面板正在搜索 “page”，列出 Page grid、Page labels、Rotate pages、Insert pages、Delete pages、Extract pages 等，并标注各自所属工具"></td>
<td width="50%"><img src="docs/images/pdfcraft-tools.png" alt="欢迎使用 PdfCraft 主界面，含推荐工具、一个最近文件、一条隐私提示，以及侧栏中的完整工具目录"></td>
</tr>
<tr>
<td align="center"><sub><kbd>⌘K</kbd> 命令面板</sub></td>
<td align="center"><sub>主界面与全部工具目录</sub></td>
</tr>
</table>

---

## 到处都能运行，数据始终属于你

- **macOS、Windows、Linux 和 FreeBSD 原生可用**，也能**在浏览器中**通过 WebAssembly 运行，同一套 Rust 代码库。Windows 构建提供 x64、x86 和 ARM64（Windows on ARM，无需模拟）；每一处 ARM64 改动都在 CI 中用 ARM64 硬件测试。
- **设计上注重隐私。** 文档永远不会离开你的机器。没有账号、没有遥测、没有云端处理。
- **引擎优先。** 解析、渲染与编辑都在可复用的库 crate 中，界面只是其上一层可替换的壳。
- **可脚本化。** `pdfcraft-cli` 工具（见 [也为智能体而建](#也为智能体而建)）覆盖检查、渲染、文本提取、编辑、合并、页面提取和拆分。健壮性扫描跑在与应用相同的引擎上。

```sh
pdfcraft-cli info  form.pdf                                  # 以 JSON 输出结构
pdfcraft-cli text  paper.pdf --page 3                        # 按阅读顺序提取文本
pdfcraft-cli edit  in.pdf --rotate 1,2:90 --delete 5 --title "Q3" --out out.pdf
```

---

## 也为智能体而建

引擎的每一项能力都能脱离图形界面调用，通过一张用 JSON Schema 描述的工具表：打开、检查、把页面渲染成 PNG、提取与查找文本、旋转/删除/移动/插入页面、编辑书签与页码标签、添加/回复/改样式/删除批注（只需说出一个短语就能高亮它）、列出并填写表单域、设置密码保护、设置元数据、撤销与重做、保存、合并、提取与拆分。三个入口共用这套工具：

- **`pdfcraft-cli run`**，用于一次性调用和 JSON 脚本：

  ```sh
  pdfcraft-cli tools                                        # 列出全部工具及其 JSON Schema
  pdfcraft-cli run text_find doc=1 query=invoice            # key=value；值按 JSON 解析
  pdfcraft-cli run --script review.json                     # 例如 comment_add {"type": "highlight", "find": "total due"}
  pdfcraft-cli run --script steps.json --root ./work        # 一次会话中执行多个步骤
  ```

  配合 `--root`，脚本各步骤读写的所有文件都会留在这个目录里，包括某一步用 `"out"` 保存的 PNG（脚本自身则从你指定的位置读取）。

- **MCP 服务器**，供 Claude 等 AI 智能体使用。**它是可选启用的：** PdfCraft 绝不会自行启动它，也不会打开任何网络端口。只有在智能体运行 `pdfcraft-cli mcp` 时才运行，通过 stdin/stdout 通信，智能体断开即停止。要启用它，请把它加进你的智能体 MCP 配置：

  ```json
  { "mcpServers": { "pdfcraft": { "command": "pdfcraft-cli", "args": ["mcp", "--root", "/path/to/your/pdfs"] } } }
  ```

  `--compact` 会缩小智能体需要读取的工具列表：`tools/list` 只返回约十个核心工具，外加 `tool_search` 和 `tool_call`，由它们查找并运行所有其他工具，因此列表占用的 token 少得多。每个工具依旧可用。

  `--root` 把智能体可读写的文件限制在一个目录内。完全不应包含该服务器的构建可以用 `cargo build -p pdfcraft-cli --no-default-features`。

- **Rust API**（`pdfcraft_automation::Automation::call`），用于嵌入到其他程序。

编辑结果保存在内存中、可撤销，直到 `doc_save`。保存到同一文件时追加一个增量更新，因此原始字节得以保留，且写入是原子的。未保存的改动绝不会被静默丢弃。

### 直接驱动应用本身

用 `pdfcraft --control /tmp/pc.json` 启动桌面应用，智能体就能看到并操作真实界面：带标签和位置的控件树（来自无障碍树）、单击、输入、按键、命令、视图选项和截图。这同样是默认关闭的。它只监听回环地址，且每个连接都必须出示写入该文件的随机令牌，而这个文件只有你能读。

```sh
pdfcraft-cli ui --control /tmp/pc.json inspect query=rotate      # 查找控件
pdfcraft-cli ui --control /tmp/pc.json click label="Organize pages"
pdfcraft-cli ui --control /tmp/pc.json key key=K modifiers='["command"]'
pdfcraft-cli ui --control /tmp/pc.json command id=comment.square   # 选一个工具，然后画：
pdfcraft-cli ui --control /tmp/pc.json drag from='[400,300]' to='[600,420]'
pdfcraft-cli ui --control /tmp/pc.json screenshot --out window.png
```

---

## 技术架构

PdfCraft 是一个由多个聚焦 crate 组成的 Cargo 工作区，分层设计使核心层永不依赖界面：

| Crate | 职责 |
|---|---|
| `pdfcraft-filters` | 全部 PDF 流过滤器（Flate、LZW、ASCII85、RunLength、预测器），编码与解码，带属性测试 |
| `pdfcraft-crypt` | 标准安全处理器：RC4、AES-128/256、修订版 2–6、权限 |
| `pdfcraft-cos` | PDF 对象层：容错解析、修复、写时复制编辑、增量与完整写入 |
| `pdfcraft-organize` | 页面操作、合并/提取/拆分、书签、页码标签、文档信息 |
| `pdfcraft-fonts` | 生成外观所需的字体度量与编码 |
| `pdfcraft-annot` | 批注：便签、文本标记、图形、墨迹和文本框的构造器与外观流；回复、状态、编辑 |
| `pdfcraft-forms` | 交互式表单：表单域模型、重新生成外观的填写、清除表单 |
| `pdfcraft-render` | 渲染、检查，以及带阅读顺序的文本提取 |
| `pdfcraft-engine` | 所有前端共用的门面：会话、编辑、撤销、保存、工具目录 |
| `pdfcraft-automation` | 智能体控制：无界面工具表、`pdfcraft-cli run`，以及可选启用的 MCP 服务器 |
| `pdfcraft-ui-egui` | 桌面端与网页版界面 |

**质量门禁。** 每一处改动都要通过同一套自动化检查：
- 格式检查，以及把警告当错误的 Clippy；
- 200 多个单元测试、属性测试和界面测试；
- crate 分层规则，以及 WebAssembly 构建检查；
- 资产许可审计。

在此之上，还有两轮真实文件语料扫描：
- **打开与渲染：** 983 个 pdf.js 测试文件中，963 个能干净地打开并渲染，0 次崩溃。
- **打开、编辑、保存往返：** 958 个成功。

输出用独立工具验证：hayro、qpdf 和 poppler。

PdfCraft 是净室实现。其行为来自 ISO 32000 规范和黑盒观察，绝不来自他人的代码。每一个图标、字体和图像都有开放许可，并记录在 [ATTRIBUTION.md](ATTRIBUTION.md) 中。

## 快速开始

```sh
git clone https://github.com/storytold/pdfcraft
cd pdfcraft
cargo run --release -p pdfcraft -- some.pdf     # 桌面应用
cargo xtask demo-pdf                              # 构建这些截图中使用的样例 PDF
cargo xtask screenshots                           # 重新生成本文档中的每一张截图
```

界面语言在 **菜单 → 编辑 → 偏好设置…** 中选择（macOS 上是 Command-逗号，其他平台是
Ctrl-逗号，没有打开文档时也可用；Auto 跟随系统语言；详见
[docs/localization.md](docs/localization.md)），并会被保存。Auto 是默认值。简体中文、
繁體中文、日本語、Русский、Español、Français 和 తెలుగు 覆盖命令、对话框、面板和键盘快捷键，
Čeština 与 Português (Brasil) 目前覆盖菜单。命令搜索同时接受译文、英文原文和稳定的命令 id；
文件名、PDF 内容、作者名、自定义动作名，以及来自引擎或操作系统的错误详情，保持其原有文字。

CJK 字体来自 [craft-fonts](https://github.com/storytold/craft-fonts)，这是一个可选构建输入，
每个正式版都会包含。要自行构建（中日韩界面文字，以及写入 PDF 的日文文字）：

```sh
git clone https://github.com/storytold/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR=../craft-fonts cargo run --release -p pdfcraft -- some.pdf
```

在不带该输入的情况下构建时，桌面端会用本机已安装的字体来绘制内嵌字体不包含的界面文字
（Windows 用微软雅黑、macOS 用苹方、Linux 用 Noto Sans CJK），因此中文或日文标签仍然可读，
不会再显示成方块；设置 `PDFCRAFT_SYSTEM_FONTS=0` 可关闭这一回退。该输入已覆盖的脚本组不会再由系统字体
绘制；Windows 上 `packaging\windows\build.ps1` 会自动接管放在仓库根目录旁的 `craft-fonts\`
（见 `docs/releasing.md`）。若该 checkout 带 `Hans` 字面——上游 `main` 已加入 `Noto Sans CJK SC`，
而正式版固定的那个提交没有——简体中文也会用这个设计字体绘制。

每个 [GitHub 发布版](https://github.com/storytold/pdfcraft/releases) 都提供 macOS、Windows、
Linux（AppImage、Flatpak、`.deb`、`.rpm` 和 tarball）、FreeBSD 和网页版的现成构建，见
[下载](#下载)。在 Gentoo 上，社区的 [::snakebyte
overlay](https://github.com/switch87/snakebyte-overlay) 把 Linux 版打包成
`app-text/pdfcraft-bin`（非 PdfCraft 团队维护）：

```sh
eselect repository add snakebyte git https://github.com/switch87/snakebyte-overlay.git
emaint sync -r snakebyte
echo 'app-text/pdfcraft-bin ~amd64' >> /etc/portage/package.accept_keywords/pdfcraft
emerge --ask app-text/pdfcraft-bin
```

日志、环境变量和其他开发说明见 [docs/development.md](docs/development.md)。

## 下一步

PdfCraft 还很年轻，但进展很快。目标是做一个可以查看、整理、批注、填写、签名和编辑 PDF 的工作台，对标 Acrobat Pro。

**目前的真实位置（2026 年 10 月）：** Acrobat Pro 离线功能的约一半已经具备（必备项的 88%），但这大约只是三分之一的工作量，因为最难的部分还在前面。

- **现在已经不错：** 查看与搜索；整理、合并与拆分；大多数批注类型；填写和创建表单（带沙箱 JavaScript）；密码、涂黑与净化；基础数字签名；打印；无障碍检查器；通过 CLI 和 MCP 进行智能体控制。
- **仍然借用：** 页面暂由 `hayro` crate 绘制，我们自己的渲染器仍在开发中。
- **薄弱或缺失：** 对既有文字的可靠编辑（尤其是 CJK）、拉丁文字之外的 OCR、Office 导入导出、签名时间戳与长期验证、PDF/A/X/UA 预检、XFA 表单，以及本地化。
- **加固中：** 模糊测试仍会在恶意文件上发现崩溃和挂起；每一个都以回归测试的形式修掉。质量尚未与 Acrobat 做过并列对比。

**接下来的顺序：** 我们自己的渲染器、加固与对标 Acrobat 的保真度测试台、编辑既有内容，然后是专业工作流（签名、OCR、Office、预检、XFA）与 1.0 打磨。

按领域的如实评估、不足之处以及我们的方向，见 **[ROADMAP.md](ROADMAP.md#honest-assessment-2026-10-05)**，其中有完整计划、进度和估算。

---

## 下载

**第一次用 PdfCraft？** 从 [getartcraft.com 的 PdfCraft 页面](https://getartcraft.com/apps/pdfcraft)下载。那是最省事的安装方式。

**想要特定构建或格式？** 在 GitHub 上，[最新发布版](https://github.com/storytold/pdfcraft/releases/latest) 列出下面所有构建，[全部发布版](https://github.com/storytold/pdfcraft/releases) 则包含更早的版本及其说明。文件名中的 `<ver>` 是版本号，`SHA256SUMS.txt` 列出每个文件的校验和。

### Windows

| 构建 | 安装包 | 便携版 |
|---|---|---|
| x64（64 位 Intel/AMD） | `pdfcraft-<ver>-windows-x64.msi` | `pdfcraft-<ver>-windows-x64-portable.zip` |
| arm64（Snapdragon 及其他 ARM PC） | `pdfcraft-<ver>-windows-arm64.msi` | `pdfcraft-<ver>-windows-arm64-portable.zip` |
| x86（32 位） | `pdfcraft-<ver>-windows-x86.msi` | `pdfcraft-<ver>-windows-x86-portable.zip` |

安装包和可执行文件均有代码签名。

便携版 zip 可在任意文件夹运行，包括 U 盘。其中的 `portable.txt` 会让设置、
日志和崩溃恢复都保存在 `pdfcraft.exe` 旁边的 `PdfCraftData` 文件夹里，因此不会写入
`%APPDATA%`；删掉该文件即可改用常规的按用户目录。

MSI 为所有用户安装，需要管理员权限。若要无人值守部署且不创建桌面快捷方式，请在提升权限的终端中运行：

```powershell
msiexec /i "pdfcraft-<ver>-windows-x64.msi" /qn /norestart INSTALLDESKTOPSHORTCUT=0
```

请按你的架构选用 MSI。不支持按用户安装的覆盖参数。

### macOS

| 构建 | 文件 | 说明 |
|---|---|---|
| 应用，通用（Apple 芯片 + Intel） | `pdfcraft-<ver>-macos-universal.dmg` | 已签名并公证 |
| 命令行工具，通用 | `pdfcraft-cli-<ver>-macos-universal.zip` | 已签名并公证 |

### Linux

| 格式 | x86_64 | aarch64（ARM64） | 说明 |
|---|---|---|---|
| AppImage | `pdfcraft-<ver>-linux-x86_64.AppImage` | `pdfcraft-<ver>-linux-aarch64.AppImage` | 随处可用；用 [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate)（`.zsync` 文件）自我更新 |
| Flatpak | `pdfcraft-<ver>-linux-x86_64.flatpak` | `pdfcraft-<ver>-linux-aarch64.flatpak` | 沙箱运行；`flatpak install --user <file>` |
| Debian/Ubuntu | `pdfcraft-<ver>-linux-x86_64.deb` | `pdfcraft-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `pdfcraft-<ver>-linux-x86_64.rpm` | `pdfcraft-<ver>-linux-aarch64.rpm` | |
| Tarball | `pdfcraft-<ver>-linux-x86_64.tar.gz` | `pdfcraft-<ver>-linux-aarch64.tar.gz` | 解包即用 |

### FreeBSD

| 构建 | 文件 |
|---|---|
| x86_64 | `pdfcraft-<ver>-freebsd-x86_64.tar.gz` |

### 网页版（WebAssembly）

| 构建 | 文件 | 说明 |
|---|---|---|
| 静态站点 | `pdfcraft-web-<ver>.zip` | 在现代浏览器中运行；托管到任意静态服务器即可 |

---

## Crafting 应用家族

PdfCraft 是 **Crafting Apps** 的一员：[ArtCraft](https://getartcraft.com/) 团队出品的免费开源创作工具，
每一个都用 Rust 从零编写，且都能独立使用。

| | 应用 | 用途 | 代码 | 了解更多 |
|:-:|---|---|---|---|
| <img src="https://raw.githubusercontent.com/storytold/photocraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.photocraft.png" alt="" width="32" height="32"> | **PhotoCraft** | 图像编辑：图层、蒙版、文字和真正的 PSD 文件 | [GitHub](https://github.com/storytold/photocraft) | [官网](https://getartcraft.com/apps/photocraft) |
| <img src="https://raw.githubusercontent.com/storytold/vectorcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.vectorcraft.png" alt="" width="32" height="32"> | **VectorCraft** | 矢量插画 | [GitHub](https://github.com/storytold/vectorcraft) | [官网](https://getartcraft.com/apps/vectorcraft) |
| <img src="https://raw.githubusercontent.com/storytold/filmcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.filmcraft.png" alt="" width="32" height="32"> | **FilmCraft** | 视频剪辑、调色与声音 | [GitHub](https://github.com/storytold/filmcraft) | [官网](https://getartcraft.com/apps/filmcraft) |
| <img src="https://raw.githubusercontent.com/storytold/lightcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.lightcraft.png" alt="" width="32" height="32"> | **LightCraft** | 照片库与 RAW 冲图 | [GitHub](https://github.com/storytold/lightcraft) | [官网](https://getartcraft.com/apps/lightcraft) |
| <img src="https://raw.githubusercontent.com/storytold/pdfcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.pdfcraft.png" alt="" width="32" height="32"> | **PdfCraft** | **阅读、整理和保护 PDF · 你在这里** | [GitHub](https://github.com/storytold/pdfcraft) | [官网](https://getartcraft.com/apps/pdfcraft) |
| <img src="https://raw.githubusercontent.com/storytold/effectcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.effectcraft.png" alt="" width="32" height="32"> | **EffectCraft** | 动态图形与视觉特效 | [GitHub](https://github.com/storytold/effectcraft) | [官网](https://getartcraft.com/apps/effectcraft) |
| <img src="https://raw.githubusercontent.com/storytold/designcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.designcraft.png" alt="" width="32" height="32"> | **DesignCraft** | 页面排版与出版 | [GitHub](https://github.com/storytold/designcraft) | [官网](https://getartcraft.com/apps/designcraft) |

还有 [**ArtCraft**](https://getartcraft.com/) 本身 —— 我们为需要真正掌控感的创作者打造的 AI 图像与视频工作室。

<br>

<p align="center">
  <a href="https://discord.gg/artcraft"><img alt="加入 ArtCraft 的 Discord 社区" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">来和我们一起创造点什么</h3>

<p align="center">
  我们的 Discord 是各类创作者聚集的地方：画画的人、摄影的人、手绘的人、剪片子的人、
  排版的人，以及还在摸索自己喜欢做什么的人。分享你正在做的东西，
  寻求帮助，告诉我们哪里坏了，或者你希望这些工具能做到什么。
  无论你用什么媒介、做了多久，这里都欢迎你。
</p>

<p align="center">
  <a href="https://discord.gg/artcraft"><b>discord.gg/artcraft</b></a> ·
  <a href="https://getartcraft.com/">getartcraft.com</a> ·
  <a href="https://getartcraft.com/apps">The Crafting Apps</a> ·
  <a href="https://getartcraft.com/apps/pdfcraft">PdfCraft</a>
</p>

---

## 许可证与致谢

PdfCraft 采用 [MIT](LICENSE-MIT) 或 [Apache-2.0](LICENSE-APACHE) 双许可，任选其一。
版权所有 (c) 2026 ArtCraft Team 与 PdfCraft 贡献者。必需的声明见 [NOTICE](NOTICE)。

随附的字体、图标、图像和其他资产各自保留其开放许可；每一项都在
[ATTRIBUTION.md](ATTRIBUTION.md) 中列出作者、来源与许可。正式版构建还会嵌入
[craft-fonts](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md)
的中日韩字体（SIL Open Font License 1.1）。

[`docs/brand/`](docs/brand/) 中的 ArtCraft 名称、字标与徽标是 ArtCraft 团队的商标，
不在本许可覆盖范围内。它们只能原样使用，只能作为本仓库和 PdfCraft 的一部分，依据
[`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) 使用。
分支和修改版必须移除它们。

<sub>Adobe、Photoshop、Illustrator、Premiere Pro、Lightroom、Acrobat、After Effects 和 InDesign 是 Adobe Inc. 在美国和/或其他国家的商标或注册商标。PdfCraft 是一个独立开源项目，与 Adobe Inc. 无隶属关系，也未获其赞助或认可；这些名称仅用于说明它所兼容的工作流。</sub>

<p align="center">
  <a href="https://getartcraft.com/"><img alt="ArtCraft" src="docs/brand/artcraft-mark.svg" width="28"></a><br>
  <sub>由 <a href="https://getartcraft.com/">ArtCraft</a> 团队与社区打造。</sub>
</p>
