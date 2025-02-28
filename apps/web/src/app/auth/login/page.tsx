import { use } from "react"
import { AuthContext } from "@/features/auth/context"

const LoginPage: React.FC = () => {
	const { login, logout } = use(AuthContext)

	const handleLog = () => {
		login({
			email: "lr@mail.com",
			password: "!abc1234ABC.",
		})
	}

	return (
		<div className="flex flex-col">
			<h1>Login</h1>
			<div className="flex flex-col">
				<button onClick={() => handleLog()}>login here</button>
				<button onClick={() => logout()}>logout here</button>
			</div>
		</div>
	)
}

export default LoginPage
