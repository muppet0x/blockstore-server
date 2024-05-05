import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import Dashboard from './Dashboard';
import axiosMock from 'axios';

jest.mock('axios');

describe('Dashboard Component Tests', () => {
  beforeEach(() => {
    process.env.REACT_APP_BACKEND_URL = 'http://localhost:5000';
  });

  it('should render dashboard component and display data correctly', async () => {
    axiosMock.get.mockResolvedValueOnce({
      data: {
        items: [
          { id: 1, name: 'Item 1', description: 'This is item 1' },
          { id: 2, name: 'Item 2', description: 'This is item 2' },
        ],
      },
    });

    render(<Dashboard />);

    expect(await screen.findByText(/Item 1/)).toBeInTheDocument();
    expect(await screen.findByText(/Item 2/)).toBeInTheDocument();
  });

  it('should handle user input correctly', () => {
    render(<Dashboard />);
    const inputElement = screen.getByPlaceholderText('Search items...');
    fireEvent.change(inputElement, { target: { value: 'Test input' } });

    expect(inputElement.value).toBe('Test input');
  });

  it('should fetch data from backend without issues', async () => {
    axiosMock.get.mockResolvedValueOnce({
      data: {
        items: [{ id: 3, name: 'Fetched Item', description: 'This is fetched from backend' }],
      },
    });

    render(<Dashboard />);

    fireEvent.click(screen.getByText('Fetch Items'));

    await waitFor(() => 
      expect(screen.getByText('Fetched Item')).toBeInTheDocument()
    );
  });

  it('should display error message when fetch fails', async () => {
    axiosMock.get.mockRejectedValueOnce(new Error('Failed to fetch'));

    render(<Dashboard />);

    fireEvent.click(screen.getByText('Retry Fetch'));

    expect(await screen.findByText('Failed to fetch')).toBeInTheDocument();
  });
});