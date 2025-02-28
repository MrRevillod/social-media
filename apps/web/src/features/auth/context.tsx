import React, { createContext, ReactNode, useState } from "react"

import { useMutation } from "@tanstack/react-query"
import { SignInSchema } from "./schemas"
import { ApiResponse, MutationFn, Nullable, User, UseStateSetter } from "@/lib/types"

import * as actions from "./actions"

type AuthContextType = {
	isAuthenticated: boolean
	user: Nullable<any>
	setIsAuthenticated: UseStateSetter<boolean>
	setUser: UseStateSetter<Nullable<any>>

	login: MutationFn<ApiResponse<User>, SignInSchema>
	logout: MutationFn<ApiResponse<null>, void>
}

export const AuthContext = createContext<AuthContextType>({} as any)

type ProviderProps = {
	children: ReactNode
}

export const AuthProvider: React.FC<ProviderProps> = ({ children }) => {
	const [isAuthenticated, setIsAuthenticated] = useState(false)
	const [user, setUser] = useState(null)

	const { mutate: login } = useMutation({
		mutationFn: (credentials: SignInSchema) => actions.logIn(credentials),
	})

	const { mutate: logout } = useMutation({
		mutationFn: () => actions.logOut(),
	})

	return (
		<AuthContext
			value={{
				user,
				setUser,
				isAuthenticated,
				setIsAuthenticated,
				login,
				logout,
			}}
		>
			{children}
		</AuthContext>
	)
}
