import { LoginSchema } from "./schemas"
import { api, protectedApi } from "@/lib/axios"
import { ApiResponse, User } from "@/lib/types"

export const logIn = async (body: LoginSchema): Promise<ApiResponse<User>> => {
    return api.post<ApiResponse<User>>("/auth/login", body).then(res => res.data)
}

export const logOut = async (): Promise<ApiResponse<null>> => {
    return protectedApi.post<ApiResponse<null>>("/auth/logout").then(res => res.data)
}

export const validateSession = async (): Promise<ApiResponse<User>> => {
    return protectedApi.post<ApiResponse<User>>("/auth/validate-session").then(res => res.data)
}
