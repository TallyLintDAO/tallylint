import { initAuth } from "@/api/auth"
import type { NNSNeuron } from "@/types/user"
import { HttpAgent } from "@dfinity/agent"
import { GovernanceCanister } from "@dfinity/nns"

export const getNNS = async (): Promise<NNSNeuron[]> => {
  let neuronList: NNSNeuron[] = []
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    const neuron = GovernanceCanister.create({
      agent: agent,
    })
    //获取授权当前pid的神经元列表
    const res = await neuron.listNeurons({ certified: false })
    if (res.length > 0) {
      for (const neuron of res) {
        if (neuron.fullNeuron) {
          const {
            id,
            accountIdentifier,
            maturityE8sEquivalent,
            stakedMaturityE8sEquivalent,
          } = neuron.fullNeuron
          const neuronData = {
            neuronId: id ? id : 0n,
            address: accountIdentifier,
            //1e8是10的八次方，除以1e8得到原数
            maturity: Number(maturityE8sEquivalent) / 1e8,
            stakedMaturity: Number(stakedMaturityE8sEquivalent) / 1e8,
            from: "hotkey",
          }
          neuronList.push(neuronData)
        }
      }
    }
  }
  return neuronList
}
