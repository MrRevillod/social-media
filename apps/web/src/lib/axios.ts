import axios from "axios"

const BASE_API_URL = import.meta.env.VITE_BASE_API_URL ?? "http://localhost/api"

const axiosOpts = {
	baseURL: BASE_API_URL,
	withCredentials: true,
}

export const api = axios.create({ ...axiosOpts })
