(async () => {
  for (let i = 0; i < 5; i++) {
    let response = await fetch("http://localhost:4000/swagger-ui", {
      method: "GET",
      headers: {
        "x-mer-key": "test",
      },
    });

    console.log(await response.text());
  }
})();
