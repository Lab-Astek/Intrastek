"use client";
import axios from "axios";
import { useState } from "react";

type Astek = {
  id: string;
  indisponibilities: any[];
  assignations: any[];
}

const TEST_ID = "2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b";

function Astek() {
  let [result, setResult] = useState<Astek>({ id: "", indisponibilities: [], assignations: [] });

  function handleClick() {
    try {

      let config = {
        method: 'get',
        maxBodyLength: Infinity,
        url: 'http://localhost:8000/asteks/2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b',
        headers: {}
      };

      axios.request(config)
        .then((response) => {
          console.log(JSON.stringify(response.data));
        })
        .catch((error) => {
          console.log(error);
        });

    }
    catch (error) {
      console.error(error);
    }
  }

  return (
    <button onClick={handleClick}>
      Click me {result.id}
    </button>
  );
}

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1>Intrastek</h1>
      <Astek />
    </main>
  );
}
