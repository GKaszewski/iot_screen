import { ModeToggle } from './mode-toggle';

const Navbar = () => {
  return (
    <nav className="flex justify-between w-full p-4 sticky top-0 z-10 bg-green-100 drop-shadow-md dark:bg-gray-800">
      <h1 className="text-2xl font-bold">IoT Screen Dashboard</h1>
      <span className="flex-1" />
      <ModeToggle />
    </nav>
  );
};

export default Navbar;
