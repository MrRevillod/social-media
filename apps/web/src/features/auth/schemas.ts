import { z } from "zod"
import { emailSchema, passwordSchema } from "../../lib/schemas/rules.schemas"

// Login schemas -----

export const signInSchema = z.object({
	email: emailSchema,
	password: z.string().min(1, { message: "Password is required" }).max(100, { message: "Password is too long" }),
})

export const signInDefaultValues = {
	email: "",
	password: "",
}

export type SignInSchema = z.infer<typeof signInSchema>

// Signup schemas -----

export const signUpSchema = z
	.object({
		username: z.string().min(1, { message: "Username is required" }).max(50, { message: "Username is too long" }),
		email: emailSchema,
		password: passwordSchema,
		confirmPassword: passwordSchema,
	})
	.refine(data => data.password === data.confirmPassword, {
		message: "Passwords do not match",
	})

export const signUpDefaultValues = {
	username: "",
	email: "",
	password: "",
	confirmPassword: "",
}

export type SignUpSchema = z.infer<typeof signUpSchema>
