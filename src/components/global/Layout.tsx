import { PropsWithChildren } from "react";
import { Navbar } from "./Navbar";
import { Footer } from "./Footer";
import { BottomNav } from "./BottomNav";
import { Toaster } from "react-hot-toast";

const Layout = ({ children }: PropsWithChildren) => {
  return (
    <div>
      <Navbar />
      {children}
      <Footer />
      <BottomNav />
      <Toaster />
    </div>
  );
};

export default Layout;
