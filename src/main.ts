// src/main.ts
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'; 


// 获取DOM元素的引用
// pc和目标开发板的信息
const pcPathInput = document.querySelector("#pc-path") as HTMLInputElement;
const pcPasswordInput = document.querySelector("#pc-password") as HTMLInputElement;
const pcIpInput = document.querySelector("#pc-ip") as HTMLInputElement;
const boardIpInput = document.querySelector("#board-ip") as HTMLInputElement;
const boardPathInput = document.querySelector("#board-path") as HTMLInputElement;
const boardUserInput = document.querySelector("#board-user") as HTMLInputElement;
const boardPasswordInput = document.querySelector("#board-password") as HTMLInputElement;

// 获取按钮和结果区域的引用
const checkBtn = document.querySelector("#check-btn") as HTMLButtonElement;
const applyBtn = document.querySelector("#apply-btn") as HTMLButtonElement;
const importBtn = document.querySelector("#import-btn") as HTMLButtonElement;
const exportBtn = document.querySelector("#export-btn") as HTMLButtonElement;
const resultArea = document.querySelector("#result-area") as HTMLPreElement;


// 监听后端事件
listen<string>('sidecar-output', (event) => {
  // 将新的日志追加到结果区域，并滚动到底部
  resultArea.textContent += event.payload + '\n';
  resultArea.scrollTop = resultArea.scrollHeight;
});


// 检查环境按钮的点击事件
checkBtn.addEventListener("click", async () => {
  console.log("检查环境按钮被点击");
  try {
    console.log("开始检查依赖");
    const message = await invoke<string>("check_dependencies"); // 交给rust端处理
    resultArea.textContent = message;
    resultArea.style.color = 'green';
  } catch (error) {
    resultArea.textContent = error as string;
    resultArea.style.color = 'red';
  }
});

// 应用配置按钮的点击事件
applyBtn.addEventListener("click", async () => {
    const pcPath = pcPathInput.value;
    const pcPassword = pcPasswordInput.value;
    const pcIp = pcIpInput.value;
    const boardIp = boardIpInput.value;
    const boardPath = boardPathInput.value;
    const boardUser = boardUserInput.value;
    const boardPassword = boardPasswordInput.value; 

    if (!pcPath || !pcPassword || !pcIp || !boardIp || !boardPath || !boardUser || !boardPassword) {
        resultArea.textContent = "错误：所有字段都必须填写！";
        resultArea.style.color = 'red';
        return;
    }
    resultArea.textContent = '正在启动自动化流程...\n';
    resultArea.style.color = 'inherit'; // 使用默认颜色

    try {
        // 调用Rust command
        await invoke("apply_nfs_share", {
            pcPath: pcPath,
            pcPassword: pcPassword,
            pcIp: pcIp,
            boardIp: boardIp,
            boardUser: boardUser,
            boardPassword: boardPassword,
            boardPath: boardPath,
        });
    } catch (error) {
        // 如果invoke本身失败（比如命令不存在）
        resultArea.textContent += `启动命令时出错: ${error}\n`;
        resultArea.style.color = 'red';
    }

    // 不返回结果，而是监听后端事件
    resultArea.textContent += "自动化流程已启动，请查看日志输出。\n";
});

// 导出按钮的点击事件
exportBtn.addEventListener("click", async () => {
    // 1. 从UI收集所有数据
    const config = {
        pc_ip: pcIpInput.value,
        pc_path: pcPathInput.value,
        pc_password: pcPasswordInput.value,
        board_ip: boardIpInput.value,
        board_user: boardUserInput.value,
        board_password: boardPasswordInput.value,
        board_path: boardPathInput.value,
    };

    try {
        // 2. 调用后端的导出命令
        await invoke("export_config", { config });
        alert("配置已成功导出！");
    } catch (error) {
        alert(`导出配置失败: ${error}`);
    }
});

// 导入按钮的点击事件
importBtn.addEventListener("click", async () => {
    try {
        // 1. 调用后端的导入命令，它会返回解析后的配置数据
        const config = await invoke<any>("import_config");

        // 2. 如果成功返回了数据，就用它来填充UI
        if (config) {
            pcIpInput.value = config.pc_ip || '';
            pcPathInput.value = config.pc_path || '';
            pcPasswordInput.value = config.pc_password || '';
            boardIpInput.value = config.board_ip || '';
            boardUserInput.value = config.board_user || '';
            boardPasswordInput.value = config.board_password || '';
            boardPathInput.value = config.board_path || '';
            alert("配置已成功导入！");
        }
    } catch (error) {
        alert(`导入配置失败: ${error}`);
    }
});
