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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Time/GET_Customers', [('url') : GlobalVariable.url, ('token') : GlobalVariable.token]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].customerId', "10")
WS.verifyElementPropertyValue(response, 'data[0].name', "aa")
WS.verifyElementPropertyValue(response, 'data[0].description', "Employee")
WS.verifyElementPropertyValue(response, 'data[0].isDeleted', "0")

WS.verifyElementPropertyValue(response, 'data[1].customerId', "5")
WS.verifyElementPropertyValue(response, 'data[1].name', "alodia")
WS.verifyElementPropertyValue(response, 'data[1].description', "Employee")
WS.verifyElementPropertyValue(response, 'data[1].isDeleted', "0")

//WS.verifyElementPropertyValue(response, 'data[6].customerId', "1")
//WS.verifyElementPropertyValue(response, 'data[6].name', "Sasuke Uchiha")
//WS.verifyElementPropertyValue(response, 'data[6].description', "")
//WS.verifyElementPropertyValue(response, 'data[6].isDeleted', "0")