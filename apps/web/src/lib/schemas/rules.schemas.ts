import { z } from "zod"

export const emailSchema = z.string().min(1, { message: "Email is required" })

export const passwordSchema = z
	.string()
	.min(8, { message: "Password is required" })
	.max(100, { message: "Password is too long" })
	.regex(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/i, {
		message:
			"Password must contain at least 8 characters, one uppercase, one lowercase, one number and one special character",
	})
