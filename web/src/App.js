import './App.css';
import React from 'react'
import { Tabs } from 'antd-mobile'

import { FILTERS, CHANNELS, COLOUR_SPACES } from './constants';

function App() {
  return (
    <div className="App">
       <Tabs defaultActiveKey='1'>
          <Tabs.Tab title='Filter' key='1'>
            {FILTERS.map((filter) => (
              <span
                className="filter"
                id={filter}
                key={filter}
              >
                {filter}
              </span>
            ))}
          </Tabs.Tab>
          <Tabs.Tab title='Coffee Latte' key='2'>
            2
          </Tabs.Tab>
          <Tabs.Tab title='Cappuccino' key='3'>
            3
          </Tabs.Tab>
          <Tabs.Tab title='Americano' key='4'>
            4
          </Tabs.Tab>
          <Tabs.Tab title='Flat White' key='5'>
            5
          </Tabs.Tab>
          <Tabs.Tab title='Caramel Macchiato' key='6'>
            6
          </Tabs.Tab>
          <Tabs.Tab title='Cafe Mocha' key='7'>
            7
          </Tabs.Tab>
        </Tabs>
    </div>
  );
}

export default App;
