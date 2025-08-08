// src/main.ts
import { invoke } from "@tauri-apps/api/core";
import { open, save } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';


// 获取DOM元素的引用
const pcPathInput = document.querySelector("#pc-path") as HTMLInputElement;
const boardIpInput = document.querySelector("#board-ip") as HTMLInputElement;
const boardPathInput = document.querySelector("#board-path") as HTMLInputElement;
const checkBtn = document.querySelector("#check-btn") as HTMLButtonElement;
const applyBtn = document.querySelector("#apply-btn") as HTMLButtonElement;
const resultArea = document.querySelector("#result-area") as HTMLPreElement;


// 检查环境按钮的点击事件
checkBtn.addEventListener("click", async () => {
  try {
    const message = await invoke<string>("check_dependencies");
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
    const boardIp = boardIpInput.value;
    const boardPath = boardPathInput.value;

    if (!pcPath || !boardIp || !boardPath) {
        resultArea.textContent = "错误：所有字段都必须填写！";
        resultArea.style.color = 'red';
        return;
    }

    try {
        const instructions = await invoke<string>("apply_nfs_share", {
            pcPath: pcPath,
            boardIp: boardIp,
            boardPath: boardPath,
        });
        resultArea.textContent = instructions;
        resultArea.style.color = 'blue';
    } catch (error) {
        resultArea.textContent = `生成命令时出错: ${error}`;
        resultArea.style.color = 'red';
    }
});