import axios from "axios"
// api: https://stat.yuku.app/swagger
export async function getNFTHistory() {
  try {
    const response = await axios.post(
      "https://stat.yuku.app/api/activity",
      {
        canister: "n5yqx-uqaaa-aaaap-aatja-cai",
        token_id: "573",
      },
      {
        headers: {
          "Content-Type": "application/json",
        },
      },
    )
    console.log("getNFTHistory", response.data)
  } catch (error) {
    console.error(error)
  }
}
