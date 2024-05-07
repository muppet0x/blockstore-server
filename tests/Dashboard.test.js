import React, { useEffect, useState } from 'react';
import axios from 'axios';

const useFetchAndCacheData = (resourceEndpoint) => {
  const [resourceData, setResourceData] = useState(null);

  useEffect(() => {
    const cachedData = sessionStorage.getItem(resourceEndpoint);
    if (cachedData !== null) {
      setResourceData(JSON.parse(cachedData));
    } else {
      axios.get(`${process.env.REACT_APP_BACKEND_URL}/${resourceEndpoint}`)
        .then(response => {
          setResourceData(response.data);
          sessionStorage.setItem(resourceEndpoint, JSON.stringify(response.data));
        })
        .catch(error => console.error('Fetching error:', error));
    }
  }, [resourceEndpoint]);

  return resourceData;
};

const Dashboard = () => {
  const fetchedItemsData = useFetchAndCacheData('items');

  return (
    <div>
      {fetchedItemsData && fetchedItemsData.items.map(item => (
        <div key={item.id}>
          <h3>{item.name}</h3>
          <p>{item.description}</p>
        </div>
      ))}
    </div>
  );
};

export default Dashboard;