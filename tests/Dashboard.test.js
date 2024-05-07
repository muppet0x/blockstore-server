import React, { useEffect, useState } from 'react';
import axios from 'axios';

const useCachedData = (endpoint) => {
  const [data, setData] = useState(null);

  useEffect(() => {
    const cache = sessionStorage.getItem(endpoint);
    if (cache !== null) {
      setData(JSON.parse(cache));
    } else {
      axios.get(`${process.env.REACT_APP_BACKEND_URL}/${endpoint}`)
        .then(response => {
          setData(response.data);
          sessionStorage.setItem(endpoint, JSON.stringify(response.data));
        })
        .catch(error => console.error(error));
    }
  }, [endpoint]);

  return data;
};

const Dashboard = () => {
  const itemsData = useCachedData('items');

  return (
    <div>
      {itemsData && itemsData.items.map(item => (
        <div key={item.id}>
          <h3>{item.name}</h3>
          <p>{item.description}</p>
        </div>
      ))}
    </div>
  );
};

export default Dashboard;