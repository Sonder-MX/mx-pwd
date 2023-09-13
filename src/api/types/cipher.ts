export interface Cipher {
  nid: string
  website: string
  username: string
  password: string
  remark?: string | null
  created: string
  updated: string
}

export interface CreateCipher {
  nid?: string
  website: string
  username: string
  password: string
  remark?: string | null
}
