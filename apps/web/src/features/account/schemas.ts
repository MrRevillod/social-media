
import { z } from "zod"
import { emailSchema, passwordSchema } from "@/lib/schemas/rules.schemas"

// Signup schemas -----

export const registerSchema = z
	.object({
		username: z.string().min(1, { message: "Username is required" }).max(50, { message: "Username is too long" }),
		email: emailSchema,
		password: passwordSchema,
		confirmPassword: passwordSchema,
	})
	.refine(data => data.password === data.confirmPassword, {
		message: "Passwords do not match",
	})

export const registerDefaultValues = {
	username: "",
	email: "",
	password: "",
	confirmPassword: "",
}

export type RegisterSchema = z.infer<typeof registerSchema>
