import React, { useEffect, useState } from 'react';
import axios from 'axios';

const useFetchResourceAndCache = (resourceEndpoint) => {
  const [resourceData, setResourceData] = useState(null);

  useEffect(() => {
    const cacheKey = resourceEndpoint;
    const cachedResourceData = sessionStorage.getItem(cacheKey);

    if (cachedResourceData !== null) {
      setResourceData(JSON.parse(cachedResourceData));
    } else {
      axios.get(`${process.env.REACT_APP_BACKEND_URL}/${resourceEndpoint}`)
        .then(response => {
          setResourceData(response.data);
          sessionStorage.setItem(cacheKey, JSON.stringify(response.data));
        })
        .catch(error => console.error('Error fetching resource:', error));
    }
  }, [resourceEndpoint]);

  return resourceData;
};

const Dashboard = () => {
  const fetchedResourceData = useFetchResourceAndCache('items');

  return (
    <div>
      {fetchedResourceData && fetchedResourceData.items.map(item => (
        <div key={item.id}>
          <h3>{item.name}</h3>
          <p>{item.description}</p>
        </div>
      ))}
    </div>
  );
};

export default Dashboard;