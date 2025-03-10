import { z } from "zod"
import { emailSchema } from "@/lib/schemas/rules.schemas"

// Login schemas -----

export const loginSchema = z.object({
	email: emailSchema,
	password: z.string().min(1, { message: "Password is required" }).max(100, { message: "Password is too long" }),
})

export const loginDefaultValues = {
	email: "",
	password: "",
}

export type LoginSchema = z.infer<typeof loginSchema>

