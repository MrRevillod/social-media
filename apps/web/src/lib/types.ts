import { UseMutateFunction } from "@tanstack/react-query"
import { Dispatch, SetStateAction } from "react"

// Utility types for types and interfaces -----

export type Nullable<T> = T | null

export type UseStateSetter<T> = Dispatch<SetStateAction<T>>

// API Call types -----

export type ApiResponse<T = unknown> = {
	data: T
	status: number
	message?: string
}

export type Conflicts = {
	conflicts: string[]
}

export type MutationFn<TData, TVariables> = UseMutateFunction<TData, Error, TVariables, unknown>

// Entity types -----

export type User = {
	id: string
	email: string
	username: string
	createdAt: string
	updatedAt: string
}
