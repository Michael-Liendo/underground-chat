const ApiRequest = () => {
  return async (url: string, options?: RequestInit) => {
    const defaultOptions = {};

    const apiUrl = `${import.meta.env.VITE_API_URL}/api${url}`;
    const requestOptions = {
      ...options,
      headers: new Headers({
        'Content-Type': 'application/json',
        ...defaultOptions,
        ...options?.headers,
      }),
    };
    const response = await fetch(apiUrl, requestOptions);
    return response;
  };
};

export default ApiRequest();
