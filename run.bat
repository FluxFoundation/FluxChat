@echo off
cd FrontEnd
npm install
npm run build
cd ..
cd backend
cargo build
cd target\debug
backend.exe
pause
