// Small wrapper around fetch for the JSON API. Errors carry the server's
// code and message.

export class ApiError extends Error {
  constructor(status, code, message) {
    super(message)
    this.status = status
    this.code = code
  }
}

export async function api(method, path, body) {
  const options = { method, headers: {}, credentials: 'same-origin' }
  if (body !== undefined) {
    options.headers['Content-Type'] = 'application/json'
    options.body = JSON.stringify(body)
  }
  const response = await fetch(`/api${path}`, options)
  const text = await response.text()
  const data = text ? JSON.parse(text) : null
  if (!response.ok) {
    throw new ApiError(response.status, data?.error ?? 'error', data?.message ?? response.statusText)
  }
  return data
}

export const get = (path) => api('GET', path)
export const post = (path, body = {}) => api('POST', path, body)
export const put = (path, body) => api('PUT', path, body)
export const del = (path) => api('DELETE', path)
