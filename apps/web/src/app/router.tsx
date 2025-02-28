import React from "react"

import ChatPage from "./chat/page"
import HomePage from "./home/page"
import LoginPage from "./auth/login/page"
import RegisterPage from "./auth/register/page"

import { Route, Routes } from "react-router-dom"

const App: React.FC = () => {
	return (
		<Routes>
			<Route path="/auth/login" element={<LoginPage />} />
			<Route path="/auth/register" element={<RegisterPage />} />
			<Route path="/chat" element={<ChatPage />} />
			<Route path="/home" element={<HomePage />} />
		</Routes>
	)
}

export default App
