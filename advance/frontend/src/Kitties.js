import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

// ref : https://github.com/SubstrateCourse/slides/tree/master/advanced/term-03/03_polkadot-js-api

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    // const kitties = []
    // setKitties(kitties)
    // updateShowKitties()
    // 根据状态，决定是否需要重新获取数据
    switch(status) {
      case '':
      case 'Current transaction status: InBlock':
        updateShowKitties();
        break;
      case 'Sending...':
      case 'Current transaction status: Ready':
      default:
        break;
    }


  }

  const updateShowKitties = () => {
    console.log('start to get kitties')

    api.query.kittiesModule.kittiesCount().then(result => {
      console.log(result)
      // 看结果是否有数据
      if (result && !result.isEmpty) {
        // 获取总的个数
        const total_kitties_count = result.words[0]
        // 初始化一个数组，内容是索引，1，2，3，4，长度是总个数
        const kittiesIndexes = [...Array(total_kitties_count).keys()]
        // 根据index 去查询所有kittey
        api.query.kittiesModule.kitties.multi(kittiesIndexes, (kittyDnas) => {
          // debugger
          console.log(kittyDnas)
          api.query.kittiesModule.owner.multi(kittiesIndexes, (owners) => {
            // debugger
            const kittiesResult = []
            kittiesIndexes.forEach((val, idx) => {
              kittiesResult.push({
                id: idx,
                dna: kittyDnas[idx].unwrap(),
                owner: owners[idx].toString()
              })
            })
            setKitties(kittiesResult)
          })
        })

        const j = 0
      }
    })
  }

  useEffect(fetchKitties, [api, keyring])
  useEffect(populateKitties, [])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
