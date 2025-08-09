// src/main.ts
import { invoke } from "@tauri-apps/api/core";


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
        // invoke现在会等待所有操作完成
        const result_logs = await invoke<string>("apply_nfs_share", {
            pcPath,
            pcPassword,
            boardIp,
            boardUser,
            boardPassword,
            boardPath,
        });
        
        // 一次性显示所有日志
        resultArea.textContent = result_logs;
        resultArea.style.color = 'green';


    } catch (error) {
        // 如果invoke返回了Err，会在这里捕获
        resultArea.textContent = error as string;
        resultArea.style.color = 'red';
    }
});