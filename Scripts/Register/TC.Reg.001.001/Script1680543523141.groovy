import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://secondhand.binaracademy.org/')

WebUI.click(findTestObject('Object Repository/Register/Page_SecondHand/a_Masuk'))

WebUI.click(findTestObject('Object Repository/Register/Page_SecondHand/a_Daftar di sini'))

WebUI.setText(findTestObject('Object Repository/Register/Page_SecondHand/input_Name_username'), 'QA 3 Y')

WebUI.setText(findTestObject('Object Repository/Register/Page_SecondHand/input_Email_useremail'), '1qa3ytester@gmail.com')

WebUI.setText(findTestObject('Object Repository/Register/Page_SecondHand/input_Password_userpassword'), 'qa3y')

WebUI.click(findTestObject('Register/Page_SecondHand/input_Password_commit'))

WebUI.click(findTestObject('Object Repository/Register/Page_SecondHand/i_Keluar_bi bi-person me-4 me-lg-0'))

if (WebUI.verifyElementText(findTestObject('Register/Page_SecondHand/Verif_NamaAkun'), 'QA', FailureHandling.OPTIONAL)) {
    WebUI.comment('Case Berhasil')
} else {
    WebUI.comment('Case Failed')
}

WebUI.closeBrowser()

