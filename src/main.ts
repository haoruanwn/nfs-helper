// src/main.ts
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'; 


// 获取DOM元素的引用
// pc和目标开发板的信息
const pcPathInput = document.querySelector("#pc-path") as HTMLInputElement;
const pcPasswordInput = document.querySelector("#pc-password") as HTMLInputElement;
const boardIpInput = document.querySelector("#board-ip") as HTMLInputElement;
const boardPathInput = document.querySelector("#board-path") as HTMLInputElement;
const boardUserInput = document.querySelector("#board-user") as HTMLInputElement;
const boardPasswordInput = document.querySelector("#board-password") as HTMLInputElement;

// 获取按钮和结果区域的引用
const checkBtn = document.querySelector("#check-btn") as HTMLButtonElement;
const applyBtn = document.querySelector("#apply-btn") as HTMLButtonElement;
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
    const boardIp = boardIpInput.value;
    const boardPath = boardPathInput.value;
    const boardUser = boardUserInput.value;
    const boardPassword = boardPasswordInput.value; 

    if (!pcPath || !pcPassword || !boardIp || !boardPath || !boardUser || !boardPassword) {
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