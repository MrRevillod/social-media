import { use } from "react"
import { AuthContext } from "@/features/auth/context"

const LoginPage: React.FC = () => {
	const { login, logout, validateSession } = use(AuthContext)

	const handleLog = () => {
		login({
			email: "lr@dev.com",
			password: "!abc1234ABC",
		})
	}

	return (
		<div className="flex flex-col">
			<h1 className="bg-red-500">Login</h1>
			<div className="flex flex-col bg-blue-300">
				<button onClick={() => handleLog()}>login here</button>
				<button onClick={() => logout()}>logout here</button>
				<button onClick={() => validateSession()}>Validate session here</button>
			</div>
		</div>
	)
}

export default LoginPage
