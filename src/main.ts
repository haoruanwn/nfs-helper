// src/main.ts
import { invoke } from "@tauri-apps/api/core";
import { open, save } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';


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
    console.log("应用配置按钮被点击");
    const pcPath = pcPathInput.value;
    const pcPassword = pcPasswordInput.value;
    const boardIp = boardIpInput.value;
    const boardPath = boardPathInput.value;
    const boardUser = boardUserInput.value;
    const boardPassword = boardPasswordInput.value; 

    if (!pcPath || !pcPassword || !boardIp || !boardPath || !boardUser || !boardPassword) {
        resultArea.textContent = "错误：所有字段都必须填写！";
        resultArea.style.color = 'red';
        console.error("错误：元素未填写全部");
        return;
    }

    try {
        const instructions = await invoke<string>("apply_nfs_share", {
            pcPath: pcPath,
            pcPassword: pcPassword,
            boardIp: boardIp,
            boardPath: boardPath,
            boardUser: boardUser,
            boardPassword: boardPassword
        });
        resultArea.textContent = instructions;
        resultArea.style.color = 'blue';
    } catch (error) {
        resultArea.textContent = `生成命令时出错: ${error}`;
        resultArea.style.color = 'red';
    }
});