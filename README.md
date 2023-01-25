# bakadict

离线的命令行日语词典

![demo](https://user-images.githubusercontent.com/24885181/212703329-a306d9fd-b8f6-473c-836f-d9b19363fab1.png)

## 安装
### Windows
#### 手动安装
从 [Release 页面](https://github.com/flaribbit/dict/releases) 下载压缩包，解压到你喜欢的位置，然后添加路径到环境变量的 `PATH` 中。

#### 使用 scoop 安装
```
scoop install https://raw.githubusercontent.com/flaribbit/bakadict/master/bakadict.json
```

### Linux, MacOS
- 克隆本仓库
- 编译并安装本程序：`cargo install --path .`
- 下载数据库：
  ```bash
  mkdir -p ~/.config/bakadict/databases
  cd ~/.config/bakadict/databases
  wget https://github.com/flaribbit/dict/releases/download/databases/jp.db
  ```

## 使用方法
- `dict 割合`
- `dict わりあい`  
  支持使用汉字或假名查询
- `dict wariai`  
  来回切换输入法很蛋疼，因此本程序还支持使用罗马音查询！
- `dict wariai -`  
  习惯上完全匹配的结果在最前面，终端输出后默认视图在底部，可以末尾添加 `-` 来反向输出查询结果。
