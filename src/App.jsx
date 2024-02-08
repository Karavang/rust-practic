import "./App.css";

function App() {
  const addFile = (e) => {
    e.preventDefault();
    const file = e.target.files[0];

    const formData = new FormData();
    formData.append("file", file);
    console.log(formData);
  };
  return (
    <>
      <div>
        <h1>Vite + React</h1>
        <div className="card">
          <input
            type="file"
            onChange={addFile}
          />
          <button>Click</button>
        </div>
      </div>
    </>
  );
}

export default App;
