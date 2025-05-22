@echo off
echo Creating deployment package...
echo.

:: Build the release
echo Building release executable...
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

:: Create deployment directory
if exist "lab-control-deployment" (
    echo Removing existing deployment directory...
    rmdir /s /q "lab-control-deployment"
)

mkdir "lab-control-deployment"
mkdir "lab-control-deployment\frontend"
mkdir "lab-control-deployment\frontend\icons"

:: Copy executable
echo Copying executable...
copy "target\release\automating_experiments.exe" "lab-control-deployment\"

:: Copy frontend files
echo Copying frontend files...
copy "frontend\index.html" "lab-control-deployment\frontend\"
copy "frontend\icons\*" "lab-control-deployment\frontend\icons\" 2>nul

:: Create batch runner
echo Creating run script...
(
echo @echo off
echo echo =====================================
echo echo  Laboratory Experiment Control System
echo echo =====================================
echo echo.
echo.
echo :: Check if the executable exists
echo if not exist "automating_experiments.exe" ^(
echo     echo ERROR: automating_experiments.exe not found!
echo     pause
echo     exit /b 1
echo ^)
echo.
echo :: Create data directory if it doesn't exist
echo if not exist "data" ^(
echo     echo Creating data directory...
echo     mkdir data
echo ^)
echo.
echo echo Starting the experiment control server...
echo echo Web interface will be available at: http://localhost:3000
echo echo Press Ctrl+C to stop the server when you're done
echo echo.
echo.
echo :: Run the application
echo automating_experiments.exe
echo.
echo echo Application has stopped.
echo pause
) > "lab-control-deployment\run_lab_control.bat"

:: Create README
echo Creating README...
(
echo Laboratory Experiment Control System - Deployment Package
echo ============================================================
echo.
echo INSTALLATION:
echo 1. Extract this folder to any location on your computer
echo 2. Ensure all instruments are connected and powered on
echo 3. Double-click "run_lab_control.bat" to start the system
echo.
echo USAGE:
echo 1. The batch file will start the server
echo 2. Open your web browser and go to: http://localhost:3000
echo 3. Click "Check Connection" for each instrument
echo 4. Select an experiment and configure parameters
echo 5. Click "Run Experiment" to start
echo.
echo Results are saved in the "data/" folder as CSV files.
echo.
echo For technical support, contact: [Your contact information]
) > "lab-control-deployment\README.txt"

echo.
echo ✓ Deployment package created successfully!
echo ✓ Location: lab-control-deployment\
echo.
echo You can now:
echo   1. Test it by running: lab-control-deployment\run_lab_control.bat
echo   2. Zip the folder for distribution
echo   3. Copy to lab computers
echo.
pause