import OSS from 'ali-oss'
import { AliOSSToken } from '../types'

// TODO: use STS token
export const makeAliOssClient = (token: AliOSSToken) => {
  return new OSS({
    region: token.region,
    accessKeyId: token.access_key_id,
    accessKeySecret: token.access_key_secret,
    bucket: token.bucket,
    timeout: 1800000,
  })
}

export const download = (url: string, filename: string) => {
  // Create a temporary anchor element
  const link = document.createElement('a')
  link.href = url

  // Set the 'download' attribute to suggest a filename
  link.setAttribute('download', filename)

  // Append the link to the document body (necessary for some older browsers)
  document.body.appendChild(link)

  // Programmatically trigger a click event on the link to start the download
  link.click()

  // Clean up: remove the link from the document body
  document.body.removeChild(link)
}

export const renameFile = (prefix: string, filename: string) => {
  const millisecondsSinceEpoch = Date.now()
  return `${prefix}/${millisecondsSinceEpoch}_${filename}`
}

export const roundTo = function (num: number | null, places: number) {
  if (num === null) {
    return null
  }
  const factor = 10 ** places
  return Math.round(num * factor) / factor
}

export const recoverFileName = (filename: string) => {
  const splits = filename.split('/', 2)
  const filenameWithTimestamp = splits[1]
  const indexOfUnderscore = filenameWithTimestamp.indexOf('_')
  return filenameWithTimestamp.substring(indexOfUnderscore + 1)
}

export const formatDate = (e: Date) => e.toLocaleDateString('en-CA')

export const addNewOption = (newOption: string, options: any[]) => {
  const option = {
    text: newOption,
    value: newOption,
  }
  options.push(option)
}
