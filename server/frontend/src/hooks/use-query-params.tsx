import { useEffect, useState } from 'react';

const useQueryParams = () => {
  const [queryParams, setQueryParams] = useState<Map<string, string>>(
    new Map()
  );
  const [location, setLocation] = useState(window.location);

  useEffect(() => {
    const searchParams = new URLSearchParams(location.search);
    const params: Map<string, string> = new Map();

    searchParams.forEach((value, key) => {
      params.set(key, value);
    });

    setQueryParams(params);
    setLocation(window.location);
  }, [window.location]);

  return queryParams;
};

export default useQueryParams;
