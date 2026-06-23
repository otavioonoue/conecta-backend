import http from "k6/http";
import { check } from "k6";


export const options = {
  stages: [
    { duration: "10s", target: 10 },
    { duration: "20s", target: 100 },
    { duration: "20s", target: 500 },
    { duration: "10s", target: 0 }
  ]
};

function randomCPF() {
  const num = Math.floor(Math.random() * 100000000000);
  return String(num).padStart(11, "0");
}

function randomPhone() {
  const ddd = Math.floor(Math.random() * 90) + 10; // 10–99
  const number = Math.floor(Math.random() * 100000000)
    .toString()
    .padStart(8, "0");

  return `55${ddd}9${number}`;
}

export default function () {
    const id = Math.floor(Math.random() * 1000000000);
  
    const payload = JSON.stringify({
      name: "Otávio",
      email: `otavio${id}@outlook.com`,
      phone: randomPhone(),
      cpf: randomCPF(),
      password: "12342234",
      address: {
        cep: "17342590",
        number: "189"
      }
    });
  
    const params = {
      headers: {
        "Content-Type": "application/json",
      },
    };
  
    const res = http.post("http://localhost:3000/user", payload, params);
  
    const ok = check(res, {
      "status 201": (r) => r.status === 201,
    });
    
    if (!ok) {
      console.log(`status=${res.status} body=${res.body}`);
    }
}