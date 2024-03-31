用于创建 MSBuild 类型的缺氧模组项目  
使用了 [@peterhaneve 缺氧模组代码仓库](https://github.com/peterhaneve/ONIMods)的模板，有些地方做了一些改动
**那么使用所谓的 MSBuild 类型的缺氧模组项目有什么好处？**
1. 首先如果使用 PLib ，把你的模组代码和 PLib 打包到一个 dll 文件中能够减少隐藏的问题
2. 缺氧模组不单单是需要一个 dll 文件，同时需要配置文件，通过使用这个模板能够自动生成配置文件
3. 在测试模组的时候，我们要把编译后的文件和配置文件移动到本地测试目录，通过使用这个模板能够自动将模组文自动移动到本地测试文件夹

## 安装
运行 oni-mod-cli-setup.exe 安装之后，需要[将安装目录添加到 Path 环境变量中](https://blog.csdn.net/chenhao0568/article/details/133167667)
## 使用
### 创建解决方案
先在文件资源管理器打开想要创建解决方案的位置，然后在选定位置打开命令行，输入以下命令：
```cmd
oni-mod-cli create sln <name>
```
其中 `name` 字段可选，可根据命令行提示完成创建流程

### 创建项目
在包含 `*.sln` 文件的目录下打开命令行，输入以下命令：
```cmd
oni-mod-cli create csproj <name>
```
`name` 字段同样可选
