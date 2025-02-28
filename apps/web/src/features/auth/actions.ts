import { api } from "@/lib/axios"
import { SignInSchema, SignUpSchema } from "./schemas"
import { ApiResponse, User } from "@/lib/types"

export const logIn = async (body: SignInSchema): Promise<ApiResponse<User>> => {
	return api.post<ApiResponse<User>>("/auth/login", body).then(res => res.data)
}

export const logOut = async (): Promise<ApiResponse<null>> => {
	return api.post<ApiResponse<null>>("/auth/logout").then(res => res.data)
}

export const signUp = async (body: SignUpSchema): Promise<ApiResponse<null>> => {
	return api.post<ApiResponse<null>>("/auth/signup", body).then(res => res.data)
}
