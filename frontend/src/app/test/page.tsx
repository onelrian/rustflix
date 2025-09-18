export default function TestPage() {
  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-4">RustFlix Test Page</h1>
        <p className="text-lg">Frontend is working correctly!</p>
        <div className="mt-8">
          <a href="/auth/login" className="text-blue-600 hover:underline">
            Go to Login
          </a>
        </div>
      </div>
    </div>
  )
}
