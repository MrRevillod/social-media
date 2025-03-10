import React, { createContext, ReactNode, useState } from "react"

import { useMutation } from "@tanstack/react-query"
import { LoginSchema } from "./schemas"
import { ApiResponse, MutationFn, Nullable, User, UseStateSetter } from "@/lib/types"

import * as actions from "./actions"

type AuthContextType = {
	isAuthenticated: boolean
	user: Nullable<User>
	setIsAuthenticated: UseStateSetter<boolean>
	setUser: UseStateSetter<Nullable<any>>

	login: MutationFn<ApiResponse<User>, LoginSchema>
	logout: MutationFn<ApiResponse<null>, void>
	validateSession: MutationFn<ApiResponse<User>, void>
}

export const AuthContext = createContext<AuthContextType>({} as any)

type ProviderProps = {
	children: ReactNode
}

export const AuthProvider: React.FC<ProviderProps> = ({ children }) => {
	const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false)
	const [user, setUser] = useState<Nullable<User>>(null)

	const { mutate: login } = useMutation({
		mutationFn: (credentials: LoginSchema) => actions.logIn(credentials),
		onSuccess: (response) => {
            setIsAuthenticated(true)
            setUser(response.data)
        },
        onError: (_response) => {
            setUser(null)
            setIsAuthenticated(false)
        }
	})

	const { mutate: validateSession } = useMutation({
        mutationFn: () => actions.validateSession(),
        onSuccess: (response) => {
            setIsAuthenticated(true)
            setUser(response.data)
        },
        onError: (_response) => {
            setUser(null)
            setIsAuthenticated(false)
        }
    })

	const { mutate: logout } = useMutation({
		mutationFn: () => actions.logOut(),
		onSettled: () => {
		    setUser(null)
		    setIsAuthenticated(false)
        }
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
				validateSession,
			}}
		>
			{children}
		</AuthContext>
	)
}
