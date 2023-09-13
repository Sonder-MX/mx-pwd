import { invoke } from "@tauri-apps/api"
import type { Cipher, CreateCipher } from "./types/cipher"
import type { Resp } from "./types/resp"

// 添加密码
export const addCipher = async (cipher: CreateCipher) => {
  return await invoke<Resp>("add_cipher", { cipher })
}

// 查询密码
export const queryCipher = async () => {
  return await invoke<Resp<Cipher[]>>("query_cipher")
}

// 删除密码
export const deleteCipher = async (nid: string) => {
  return await invoke<Resp>("delete_cipher", { nid })
}

// 更新密码
export const updateCipher = async (cipher: Cipher) => {
  return await invoke<Resp>("update_cipher", { cipher })
}

// 根据nid查询密码
export const queryCipherByNid = async (nid: string) => {
  return await invoke<Resp<Cipher>>("query_cipher_by_nid", { nid })
}
